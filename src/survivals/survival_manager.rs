use crate::debug::d;
use crate::survivals::scenario_txt_builder::build_ending_txt;
use crate::survivals::scenario_txt_builder::build_scenario_txt;
use crate::survivals::story_csv_builder::build_story_csv;
use crate::swr::*;
use crate::union_cast;
use encoding_rs::SHIFT_JIS;
use regex::Regex;
use std::ffi::CStr;
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
    original_load_txt: 0,
    original_load_csv: 0,
    original_save_clear_operator: 0u8,
    original_save_result_operators: None,
};

unsafe fn tamper_bytes(addr: u32, data: &[u8]) -> Vec<u8> {
    let mut old = Vec::new();
    for i in 0..data.len() {
        let ptr = (addr + i as u32) as *mut u8;
        old.push(*ptr);
        *ptr = data[i];
    }
    old
}

unsafe fn load_txt(obj: LPVOID, file_name: *const c_char) -> BOOL {
    union_cast!(extern "fastcall" fn(obj: LPVOID, file_name: *const c_char) -> BOOL)(
        SURVIVAL_MANAGER.original_load_txt,
    )(obj, file_name)
}

unsafe fn load_csv(obj: LPVOID, file_name: *const c_char) -> BOOL {
    union_cast!(extern "fastcall" fn(obj: LPVOID, file_name: *const c_char) -> BOOL)(
        SURVIVAL_MANAGER.original_load_csv,
    )(obj, file_name)
}

unsafe fn new_text_object(text_size: DWORD) -> *mut c_void {
    union_cast!(extern "cdecl" fn(text_size: DWORD) -> *mut c_void)(0x00664F9A)(text_size)
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
    match re.captures(&file_name_str) {
        Some(cap) => {
            let csv = build_scenario_txt(&cap[1], cap[2].parse().unwrap());
            unsafe {
                *(obj as *mut u32) = create_text_object(&csv) as u32;
            }
            return TRUE;
        }
        None => {}
    }
    let re = Regex::new(r"data/scenario/.+/ed\.txt").unwrap();
    match re.captures(&file_name_str) {
        Some(_) => {
            let csv = build_ending_txt();
            unsafe {
                *(obj as *mut u32) = create_text_object(&csv) as u32;
            }
            return TRUE;
        }
        None => {}
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
    original_load_txt: u32,
    original_load_csv: u32,
    original_save_clear_operator: u8,
    original_save_result_operators: Option<Vec<u8>>,
}

impl SurvivalManager {
    pub fn is_active(&self) -> bool {
        self.original_load_txt != 0
    }

    pub fn tamper(&mut self) {
        let mut old: DWORD = 0;
        unsafe {
            VirtualProtect(text_Offset, text_Size, PAGE_EXECUTE_WRITECOPY, &mut old);
            self.original_load_txt = TamperNearJmpOpr(0x4059F3, on_load_txt as DWORD);
            self.original_load_csv = TamperNearJmpOpr(0x40EB63, on_load_csv as DWORD);
            self.original_save_clear_operator = tamper_bytes(0x42D62B, &[0xEBu8])[0];
            self.original_save_result_operators =
                Some(tamper_bytes(0x43EB5B, &0x9090909090u64.to_be_bytes()[3..8])); // リザルト保存を抑止
            VirtualProtect(text_Offset, text_Size, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), NULL, 0);
        }
    }

    pub fn restore(&mut self) {
        let mut old: DWORD = 0;
        unsafe {
            VirtualProtect(text_Offset, text_Size, PAGE_EXECUTE_WRITECOPY, &mut old);
            TamperNearJmpOpr(0x4059F3, self.original_load_txt);
            TamperNearJmpOpr(0x40EB63, self.original_load_csv);
            tamper_bytes(0x42D62B, &[self.original_save_clear_operator]);
            tamper_bytes(
                0x43EB5B,
                self.original_save_result_operators.as_ref().unwrap(),
            );
            VirtualProtect(text_Offset, text_Size, old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), NULL, 0);
        }
        self.original_load_txt = 0;
        self.original_load_csv = 0;
        self.original_save_clear_operator = 0;
        self.original_save_result_operators = None;
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
