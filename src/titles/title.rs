use crate::survivals::survival_manager::SURVIVAL_MANAGER;
use crate::swr::*;
use std::mem::transmute;
use winapi::shared::minwindef::LPVOID;

unsafe fn c_title_create(this: LPVOID) -> LPVOID {
    let func: extern "thiscall" fn(this: LPVOID) -> LPVOID = transmute(ORIGINAL_C_TITLE_CREATE);
    func(this)
}

static mut ORIGINAL_C_TITLE_CREATE: usize = 0;

extern "thiscall" fn c_title_on_create(this: LPVOID) -> LPVOID {
    unsafe {
        if SURVIVAL_MANAGER.is_active() {
            SURVIVAL_MANAGER.restore();
        }
        c_title_create(this)
    }
}

pub unsafe fn tamper_text() {
    ORIGINAL_C_TITLE_CREATE = TamperNearJmpOpr(CTitle_Creater, c_title_on_create as usize);
}
