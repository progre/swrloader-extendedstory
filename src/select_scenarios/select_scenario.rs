use crate::select_scenarios::text_texture::TextTexture;
use crate::select_scenarios::text_texture::TextTextureFactory;
use crate::select_scenarios::texture_renderer::render_texture;
use crate::survivals::survival_manager::SURVIVAL_MANAGER;
use crate::swr::*;
use crate::union_cast;
use std::os::raw::c_void;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;

unsafe fn c_select_scenario_create(this: LPVOID) -> LPVOID {
    union_cast!(extern "thiscall" fn(this: LPVOID) -> LPVOID)(ORIGINAL_C_SELECT_SCENARIO_CREATE)(
        this,
    )
}

unsafe fn c_select_scenario_destruct(this: LPVOID, dyn_: DWORD) -> LPVOID {
    union_cast!(extern "thiscall" fn(this: LPVOID, dyn_: DWORD) -> LPVOID)(
        ORIGINAL_C_SELECT_SCENARIO_DESTRUCT,
    )(this, dyn_)
}

unsafe fn c_select_scenario_update(this: *const c_void) -> i32 {
    union_cast!(extern "thiscall" fn(this: *const c_void) -> i32)(ORIGINAL_C_SELECT_SCENARIO_UPDATE)(
        this,
    )
}

unsafe fn c_select_scenario_render(this: LPVOID) -> DWORD {
    union_cast!(extern "thiscall" fn(this: LPVOID) -> DWORD)(ORIGINAL_C_SELECT_SCENARIO_RENDER)(
        this,
    )
}

static mut ORIGINAL_C_SELECT_SCENARIO_CREATE: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_DESTRUCT: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_UPDATE: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_RENDER: DWORD = 0;
static mut TEXTURE: Option<TextTexture> = None;

extern "thiscall" fn c_select_scenario_on_create(this: LPVOID) -> LPVOID {
    let texture = TextTextureFactory::new().create_texture(250, 50, "Survival Mode");
    unsafe {
        TEXTURE = Some(texture);
        c_select_scenario_create(this)
    }
}

unsafe extern "thiscall" fn c_select_scenario_on_destruct(this: LPVOID, dyn_: DWORD) -> LPVOID {
    TEXTURE = None;

    c_select_scenario_destruct(this, dyn_)
}

extern "thiscall" fn c_select_scenario_on_update(this: *const c_void) -> i32 {
    let key4 = unsafe {
        let unknown_obj1 = *((this as u32 + 0x0c) as *const u32);
        *((unknown_obj1 + 0x4c) as *const u32)
    };
    if key4 == 1 {
        unsafe {
            if !SURVIVAL_MANAGER.is_active() {
                SURVIVAL_MANAGER.tamper();
            } else {
                SURVIVAL_MANAGER.restore();
            }
        }
    }
    unsafe { c_select_scenario_update(this) }
}

unsafe extern "thiscall" fn c_select_scenario_on_render(this: LPVOID) -> DWORD {
    let ret = c_select_scenario_render(this);

    if !SURVIVAL_MANAGER.is_active() {
        return ret;
    }
    render_texture(&TEXTURE.as_ref().unwrap());
    ret
}

pub unsafe fn tamper_text() {
    ORIGINAL_C_SELECT_SCENARIO_CREATE = TamperNearJmpOpr(
        CSelectScenario_Creater,
        c_select_scenario_on_create as DWORD,
    );
}

pub unsafe fn tamper_r_data() {
    ORIGINAL_C_SELECT_SCENARIO_DESTRUCT = TamperDword(
        vtbl_CSelectScenario + 0x00,
        c_select_scenario_on_destruct as DWORD,
    );
    ORIGINAL_C_SELECT_SCENARIO_UPDATE = TamperDword(
        vtbl_CSelectScenario + 0x04,
        c_select_scenario_on_update as DWORD,
    );
    ORIGINAL_C_SELECT_SCENARIO_RENDER = TamperDword(
        vtbl_CSelectScenario + 0x08,
        c_select_scenario_on_render as DWORD,
    );
}
