use crate::debug::d;
use crate::survivals::scenario_txt_builder::build_ending_txt;
use crate::survivals::scenario_txt_builder::build_scenario_txt;
use crate::survivals::story_csv_builder::build_story_csv;
use crate::survivals::tamperer::Tamperer;
use crate::swr::*;
use encoding_rs::SHIFT_JIS;
use regex::Regex;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_char;
use std::os::raw::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::TRUE;
use winapi::shared::ntdef::NULL;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::processthreadsapi::FlushInstructionCache;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winnt::PAGE_EXECUTE_WRITECOPY;

pub static mut SURVIVAL_MANAGER: SurvivalManager = SurvivalManager {
    tamperer_load_txt: None,
    tamperer_load_csv: None,
    tamperer_save_clear: None,
    tamperer_add_cards: None,
    tamperer_save_result: None,
    tamperer_save_replay_interrupted: None,
    tamperer_save_replay_clear: None,
};

unsafe fn load_txt(obj: LPVOID, file_name: *const c_char) -> BOOL {
    let func: extern "fastcall" fn(obj: LPVOID, file_name: *const c_char) -> BOOL = transmute(
        SURVIVAL_MANAGER
            .tamperer_load_txt
            .as_ref()
            .unwrap()
            .jmp_target_addr(),
    );
    func(obj, file_name)
}

unsafe fn load_csv(obj: LPVOID, file_name: *const c_char) -> BOOL {
    let func: extern "fastcall" fn(obj: LPVOID, file_name: *const c_char) -> BOOL = transmute(
        SURVIVAL_MANAGER
            .tamperer_load_csv
            .as_ref()
            .unwrap()
            .jmp_target_addr(),
    );
    func(obj, file_name)
}

unsafe fn new_text_object(text_size: DWORD) -> *mut c_void {
    let func: extern "cdecl" fn(text_size: DWORD) -> *mut c_void = transmute(0x00664F9A);
    func(text_size)
}

unsafe fn create_text_object(text: &str) -> *const c_void {
    let text_array = SHIFT_JIS.encode(&text).0;
    // 謎構造体をnewして、先頭にテキストデータをセット
    let csv_obj = new_text_object(text_array.len() as DWORD);
    for i in 0..text_array.len() {
        *((csv_obj as u32 + i as u32) as *mut c_char) = text_array[i] as i8;
    }
    csv_obj
}

extern "fastcall" fn on_load_txt(obj: LPVOID, file_name: *const c_char) -> BOOL {
    let cstr = unsafe { CStr::from_ptr(file_name) };
    let file_name_str = SHIFT_JIS.decode(cstr.to_bytes()).0;
    d(&format!("read txt: {}", file_name_str));

    let re = Regex::new(r"data/scenario/(.+)/(\d+)\.txt").unwrap();
    if let Some(cap) = re.captures(&file_name_str) {
        let csv = build_scenario_txt(&cap[1], cap[2].parse().unwrap());
        unsafe {
            *(obj as *mut u32) = create_text_object(&csv) as u32;
        }
        return TRUE;
    }
    let re = Regex::new(r"data/scenario/.+/ed\.txt").unwrap();
    if re.captures(&file_name_str).is_some() {
        let csv = build_ending_txt();
        unsafe {
            *(obj as *mut u32) = create_text_object(&csv) as u32;
        }
        return TRUE;
    }

    unsafe { load_txt(obj, file_name) }
}

extern "fastcall" fn on_load_csv(obj: LPVOID, file_name: *const c_char) -> BOOL {
    let cstr = unsafe { CStr::from_ptr(file_name) };
    let file_name_str = SHIFT_JIS.decode(cstr.to_bytes()).0;
    d(&format!("read csv: {}", file_name_str));

    let re = Regex::new(r"data/csv/.+/story.csv").unwrap();
    if re.is_match(&file_name_str) {
        let csv = build_story_csv();
        unsafe {
            *(obj as *mut u32) = create_text_object(&csv) as u32;
        }
        return TRUE;
    }

    unsafe { load_csv(obj, file_name) }
}

pub struct SurvivalManager {
    tamperer_load_txt: Option<Tamperer>,
    tamperer_load_csv: Option<Tamperer>,
    tamperer_save_clear: Option<Tamperer>,
    tamperer_add_cards: Option<Tamperer>,
    tamperer_save_result: Option<Tamperer>,
    tamperer_save_replay_interrupted: Option<Tamperer>,
    tamperer_save_replay_clear: Option<Tamperer>,
}

impl SurvivalManager {
    pub fn is_active(&self) -> bool {
        self.tamperer_load_txt.is_some()
    }

    pub fn tamper(&mut self) {
        let mut old: DWORD = 0;
        unsafe {
            VirtualProtect(text_Offset, text_Size, PAGE_EXECUTE_WRITECOPY, &mut old);

            self.tamperer_load_txt =
                Some(Tamperer::near_jmp_operator(0x4059F3, on_load_txt as usize));
            self.tamperer_load_csv =
                Some(Tamperer::near_jmp_operator(0x40EB63, on_load_csv as usize));
            self.tamperer_save_clear = Some(Tamperer::byte(0x42D62B, 0xEBu8));
            self.tamperer_add_cards = Some(Tamperer::bytes(0x43EA8C, &0x90E9u16.to_be_bytes())); // 獲得カードを無効化
            self.tamperer_save_result = Some(Tamperer::bytes(
                0x43EB5B,
                &0x9090909090u64.to_be_bytes()[3..8],
            )); // リザルト保存を抑止
            self.tamperer_save_replay_interrupted = Some(Tamperer::byte(0x43EE11, 0xEBu8)); // ストーリー中断リプレイ保存抑止
            self.tamperer_save_replay_clear = Some(Tamperer::byte(0x4433DF, 0xEBu8)); // ストーリー完了リプレイ抑止

            VirtualProtect(text_Offset, text_Size, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), NULL, 0);
        }
    }

    pub fn restore(&mut self) {
        let mut old: DWORD = 0;
        unsafe {
            VirtualProtect(text_Offset, text_Size, PAGE_EXECUTE_WRITECOPY, &mut old);

            self.tamperer_load_txt = None;
            self.tamperer_load_csv = None;
            self.tamperer_save_clear = None;
            self.tamperer_add_cards = None;
            self.tamperer_save_result = None;
            self.tamperer_save_replay_interrupted = None;
            self.tamperer_save_replay_clear = None;

            VirtualProtect(text_Offset, text_Size, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), NULL, 0);
        }
    }
}

impl Drop for SurvivalManager {
    fn drop(&mut self) {
        if self.is_active() {
            self.restore();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bytes() {
        assert_eq!(
            &[0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8],
            &0x0102030405u64.to_be_bytes()[3..8]
        );
    }
}
