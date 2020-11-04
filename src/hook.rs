use crate::select_scenarios::select_scenario;
use crate::swr::*;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::ntdef::NULL;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::processthreadsapi::FlushInstructionCache;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winnt::PAGE_EXECUTE_WRITECOPY;

pub fn hook(_module: HMODULE) {
    unsafe {
        let mut old: DWORD = 0;
        VirtualProtect(text_Offset, text_Size, PAGE_EXECUTE_WRITECOPY, &mut old);
        select_scenario::tamper_text();
        VirtualProtect(text_Offset, text_Size, old, &mut old);

        VirtualProtect(rdata_Offset, rdata_Size, PAGE_EXECUTE_WRITECOPY, &mut old);
        select_scenario::tamper_r_data();
        VirtualProtect(rdata_Offset, rdata_Size, old, &mut old);

        FlushInstructionCache(GetCurrentProcess(), NULL, 0);
    }
}
