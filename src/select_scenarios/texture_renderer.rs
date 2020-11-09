use crate::select_scenarios::text_texture::TextTexture;
use crate::swr::*;
use std::mem::size_of;
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
    fn new_2d(x: FLOAT, y: FLOAT, color: D3DCOLOR, u: FLOAT, v: FLOAT) -> Self {
        SWRVERTEX {
            x,
            y,
            z: 0.0,
            rhw: 1.0,
            color,
            u,
            v,
        }
    }
}

pub unsafe fn render_texture(tex: &TextTexture, width: f32, height: f32) {
    if CRenderer_Begin(g_renderer) == TRUE {
        let vertices = [
            SWRVERTEX::new_2d(0.0, 0.0, 0xffffffff, 0.0, 0.0),
            SWRVERTEX::new_2d(width, 0.0, 0xffffffff, 1.0, 0.0),
            SWRVERTEX::new_2d(width, height, 0xffffffff, 1.0, 1.0),
            SWRVERTEX::new_2d(0.0, height, 0xffffffff, 0.0, 1.0),
        ];
        let vertices2 = [
            SWRVERTEX::new_2d(0.0, 0.0, 0x60000000, 0.0, 0.0),
            SWRVERTEX::new_2d(width, 0.0, 0x60000000, 1.0, 0.0),
            SWRVERTEX::new_2d(width, height - 15.0, 0x60000000, 1.0, 1.0),
            SWRVERTEX::new_2d(0.0, height - 15.0, 0x60000000, 0.0, 1.0),
        ];

        CTextureManager_SetTexture(g_textureMgr, NULL as DWORD, 0);
        g_pd3dDev().DrawPrimitiveUP(
            D3DPT_TRIANGLEFAN,
            2,
            vertices2.as_ptr() as LPVOID,
            size_of::<SWRVERTEX>() as DWORD,
        );

        CTextureManager_SetTexture(g_textureMgr, tex.tex_id(), 0);
        g_pd3dDev().DrawPrimitiveUP(
            D3DPT_TRIANGLEFAN,
            2,
            vertices.as_ptr() as LPVOID,
            size_of::<SWRVERTEX>() as DWORD,
        );

        CRenderer_End(g_renderer);
    }
}
