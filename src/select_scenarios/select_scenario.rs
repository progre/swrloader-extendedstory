use crate::select_scenarios::text_texture::TextTexture;
use crate::select_scenarios::text_texture::TextTextureFactory;
use crate::select_scenarios::texture_renderer::render_texture;
use crate::survivals::survival_manager::SURVIVAL_MANAGER;
use crate::swr::*;
use std::mem::transmute;
use std::os::raw::c_void;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;

const VERSION: &str = env!("CARGO_PKG_VERSION");

static mut ORIGINAL_C_SELECT_SCENARIO_CREATE: usize = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_DESTRUCT: usize = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_UPDATE: usize = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_RENDER: usize = 0;
static mut TEXTURE: Option<TextTexture> = None;

// 17: 宣言
// 1a: 点 1b-1d 空振り 1e: 着地
// 27: 選択
// 28: 決定
// 29: キャンセル
// 2c: 決着
// 2d: チャージ
// 30: 萃夢想オプション
// 35: 宣言
// 36: スキカ
// 37: 霊撃
// 39: ベル
unsafe fn play_sound(idx: i32) {
    let func: extern "C" fn(idx: i32) = transmute(0x439DC0);
    func(idx)
}

unsafe fn c_select_scenario_create(this: LPVOID) -> LPVOID {
    let func: extern "thiscall" fn(this: LPVOID) -> LPVOID =
        transmute(ORIGINAL_C_SELECT_SCENARIO_CREATE);
    func(this)
}

unsafe fn c_select_scenario_destruct(this: LPVOID, dyn_: DWORD) -> LPVOID {
    let func: extern "thiscall" fn(this: LPVOID, dyn_: DWORD) -> LPVOID =
        transmute(ORIGINAL_C_SELECT_SCENARIO_DESTRUCT);
    func(this, dyn_)
}

unsafe fn c_select_scenario_update(this: *const c_void) -> i32 {
    let func: extern "thiscall" fn(this: *const c_void) -> i32 =
        transmute(ORIGINAL_C_SELECT_SCENARIO_UPDATE);
    func(this)
}

unsafe fn c_select_scenario_render(this: LPVOID) -> DWORD {
    let func: extern "thiscall" fn(this: LPVOID) -> DWORD =
        transmute(ORIGINAL_C_SELECT_SCENARIO_RENDER);
    func(this)
}

extern "thiscall" fn c_select_scenario_on_create(this: LPVOID) -> LPVOID {
    let texture =
        TextTextureFactory::new().create_texture(320, 50, &format!("Survival Mode v{}", VERSION));
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
                play_sound(0x30);
            } else {
                SURVIVAL_MANAGER.restore();
                play_sound(0x29);
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
    render_texture(&TEXTURE.as_ref().unwrap(), 320.0, 50.0);
    ret
}

pub unsafe fn tamper_text() {
    ORIGINAL_C_SELECT_SCENARIO_CREATE = TamperNearJmpOpr(
        CSelectScenario_Creater,
        c_select_scenario_on_create as usize,
    );
}

pub unsafe fn tamper_r_data() {
    ORIGINAL_C_SELECT_SCENARIO_DESTRUCT =
        TamperDword(vtbl_CSelectScenario, c_select_scenario_on_destruct as usize);
    ORIGINAL_C_SELECT_SCENARIO_UPDATE = TamperDword(
        vtbl_CSelectScenario + 0x04,
        c_select_scenario_on_update as usize,
    );
    ORIGINAL_C_SELECT_SCENARIO_RENDER = TamperDword(
        vtbl_CSelectScenario + 0x08,
        c_select_scenario_on_render as usize,
    );
}
