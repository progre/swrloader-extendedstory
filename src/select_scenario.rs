use crate::debug::d;
use crate::survivals::survival_manager::SURVIVAL_MANAGER;
use crate::survivals::text_texture::TextTexture;
use crate::survivals::text_texture::TextTextureFactory;
use crate::swr::*;
use crate::{union_cast, Ccall};
use std::mem::size_of;
use std::os::raw::c_void;
use winapi::shared::d3d9types::D3DCOLOR;
use winapi::shared::d3d9types::D3DPT_TRIANGLEFAN;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::FLOAT;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::TRUE;
use winapi::shared::ntdef::NULL;

#[repr(C)]
struct SWRVERTEX {
    x: FLOAT,
    y: FLOAT,
    z: FLOAT,
    rhw: FLOAT,
    color: D3DCOLOR,
    u: FLOAT,
    v: FLOAT,
}

impl SWRVERTEX {
    fn new(x: FLOAT, y: FLOAT, z: FLOAT, rhw: FLOAT, color: D3DCOLOR, u: FLOAT, v: FLOAT) -> Self {
        SWRVERTEX {
            x: x,
            y: y,
            z: z,
            rhw: rhw,
            color: color,
            u: u,
            v: v,
        }
    }

    fn new_2d(x: FLOAT, y: FLOAT, color: D3DCOLOR, u: FLOAT, v: FLOAT) -> Self {
        SWRVERTEX {
            x: x,
            y: y,
            z: 0.0f32,
            rhw: 1.0f32,
            color: color,
            u: u,
            v: v,
        }
    }
}

unsafe fn c_select_scenario_create(this: LPVOID) -> LPVOID {
    Ccall!(
        ORIGINAL_C_SELECT_SCENARIO_CREATE,
        extern "thiscall" fn(this: LPVOID) -> LPVOID
    )(this)
}

unsafe fn c_select_scenario_destruct(this: LPVOID, dyn_: DWORD) -> LPVOID {
    Ccall!(
        ORIGINAL_C_SELECT_SCENARIO_DESTRUCT,
        extern "thiscall" fn(this: LPVOID, dyn_: DWORD) -> LPVOID
    )(this, dyn_)
}

unsafe fn c_select_scenario_update(this: *const c_void) -> i32 {
    union_cast!(extern "thiscall" fn(this: *const c_void) -> i32)(ORIGINAL_C_SELECT_SCENARIO_UPDATE)(
        this,
    )
}

unsafe fn c_select_scenario_render(this: LPVOID) -> DWORD {
    Ccall!(
        ORIGINAL_C_SELECT_SCENARIO_RENDER,
        extern "thiscall" fn(this: LPVOID) -> DWORD
    )(this)
}

static mut ORIGINAL_C_SELECT_SCENARIO_SIZE: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_CREATE: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_DESTRUCT: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_UPDATE: DWORD = 0;
static mut ORIGINAL_C_SELECT_SCENARIO_RENDER: DWORD = 0;
static mut TEXTURE: Option<TextTexture> = None;

unsafe extern "thiscall" fn c_select_scenario_on_create(this: LPVOID) -> LPVOID {
    d("c_select_scenario_on_create");

    c_select_scenario_create(this);

    TEXTURE = Some(TextTextureFactory::new().create_texture(250, 50, "Survival Mode"));

    return this;
}

unsafe extern "thiscall" fn c_select_scenario_on_destruct(this: LPVOID, dyn_: DWORD) -> LPVOID {
    TEXTURE = None;

    return c_select_scenario_destruct(this, dyn_);
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
    if CRenderer_Begin(g_renderer) == TRUE {
        let vertices = [
            SWRVERTEX::new_2d(0.0f32, 0.0f32, 0xffffffff, 0.0f32, 0.0f32),
            SWRVERTEX::new_2d(250.0f32, 0.0f32, 0xffffffff, 1.0f32, 0.0f32),
            SWRVERTEX::new_2d(250.0f32, 50.0f32, 0xffffffff, 1.0f32, 1.0f32),
            SWRVERTEX::new_2d(0.0f32, 50.0f32, 0xffffffff, 0.0f32, 1.0f32),
        ];
        let vertices2 = [
            SWRVERTEX::new_2d(0.0f32, 0.0f32, 0x60000000, 0.0f32, 0.0f32),
            SWRVERTEX::new_2d(250.0f32, 0.0f32, 0x60000000, 1.0f32, 0.0f32),
            SWRVERTEX::new_2d(250.0f32, 40.0f32, 0x60000000, 1.0f32, 1.0f32),
            SWRVERTEX::new_2d(0.0f32, 40.0f32, 0x60000000, 0.0f32, 1.0f32),
        ];

        CTextureManager_SetTexture(g_textureMgr, NULL as DWORD, 0);
        g_pd3dDev().DrawPrimitiveUP(
            D3DPT_TRIANGLEFAN,
            2,
            vertices2.as_ptr() as LPVOID,
            size_of::<SWRVERTEX>() as DWORD,
        );

        CTextureManager_SetTexture(g_textureMgr, TEXTURE.as_ref().unwrap().tex_id(), 0);
        g_pd3dDev().DrawPrimitiveUP(
            D3DPT_TRIANGLEFAN,
            2,
            vertices.as_ptr() as LPVOID,
            size_of::<SWRVERTEX>() as DWORD,
        );

        CRenderer_End(g_renderer);
    }
    return ret;
}

pub unsafe fn tamper_text() {
    let msg = format!(
        "c_select_scenario_on_create {}",
        c_select_scenario_on_create as DWORD
    );
    d(&msg);
    ORIGINAL_C_SELECT_SCENARIO_SIZE = *CSelectScenario_Size;
    *CSelectScenario_Size += 4;
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
