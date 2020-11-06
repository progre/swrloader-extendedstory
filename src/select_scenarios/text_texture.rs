use crate::debug::d;
use crate::swr::g_textureMgr;
use crate::swr::CTextureManager_CreateTextTexture;
use crate::swr::CTextureManager_Remove;
use crate::swr::SWRFont_Create;
use crate::swr::SWRFont_Destruct;
use crate::swr::SWRFont_SetIndirect;
use crate::swr::SWRFONTDESC;
use encoding_rs::SHIFT_JIS;
use std::ffi::CString;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;

fn create_font_desc() -> SWRFONTDESC {
    let mut face_name = [0u8; 0x100];
    let face_name_cstr = CString::new("MS Gothic").unwrap();
    let face_name_bytes = face_name_cstr.as_bytes_with_nul();
    face_name[..(face_name_bytes.len())].clone_from_slice(face_name_bytes);

    SWRFONTDESC {
        FaceName: face_name,
        R1: 0xFF,
        R2: 0xA0,
        G1: 0xFF,
        G2: 0xA0,
        B1: 0xFF,
        B2: 0xFF,
        Height: 0x20,
        Weight: 300,
        Italic: 0,
        Shadow: 1,
        UseOffset: 0,
        BufferSize: 100000,
        OffsetX: 0,
        OffsetY: 0,
        CharSpaceX: 0,
        CharSpaceY: 2,
    }
}

struct SWRFont {
    memory: [u8; 0x1A4], // 多くとも 0x1A4
}

impl SWRFont {
    pub fn new() -> Self {
        let memory = [0u8; 0x1A4];
        let mut zelf = SWRFont { memory };
        let font_desc = create_font_desc();
        unsafe {
            let ptr = zelf.as_mut_ptr();
            SWRFont_Create(ptr);
            SWRFont_SetIndirect(ptr, &font_desc as *const SWRFONTDESC as LPVOID);
        }
        zelf
    }

    pub unsafe fn as_mut_ptr(&mut self) -> LPVOID {
        &mut self.memory as *mut [u8; 0x1a4] as LPVOID
    }

    pub unsafe fn as_ptr(&self) -> LPVOID {
        &self.memory as *const [u8; 0x1a4] as LPVOID
    }
}

impl Drop for SWRFont {
    fn drop(&mut self) {
        unsafe {
            SWRFont_Destruct(self.as_ptr());
            d("font destructed");
        }
    }
}

pub struct TextTextureFactory {
    font: SWRFont,
}

impl TextTextureFactory {
    pub fn new() -> Self {
        TextTextureFactory {
            font: SWRFont::new(),
        }
    }

    pub fn create_texture(&self, width: u32, height: u32, text: &str) -> TextTexture {
        TextTexture::new(width, height, text, &self.font)
    }
}

pub struct TextTexture {
    tex_id: DWORD,
}

impl TextTexture {
    fn new(width: u32, height: u32, text: &str, font: &SWRFont) -> Self {
        let zero_ended_string = format!("{}\0", text);
        let sjis = SHIFT_JIS.encode(&zero_ended_string).0;
        // as_ptr() 使用中に dropしないよう注意
        let mut tex_id = 0;
        unsafe {
            CTextureManager_CreateTextTexture(
                g_textureMgr,
                &mut tex_id,
                sjis.as_ptr() as *const i8,
                font.as_ptr(),
                width,
                height,
            );
        }
        TextTexture { tex_id }
    }

    pub fn tex_id(&self) -> DWORD {
        self.tex_id
    }
}

impl Drop for TextTexture {
    fn drop(&mut self) {
        unsafe {
            CTextureManager_Remove(g_textureMgr, self.tex_id);
            d("texture destructed");
        }
    }
}
