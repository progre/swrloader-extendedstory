#![feature(abi_thiscall)]

mod debug;
mod hook;
mod select_scenarios;
mod survivals;
mod swr;

use crate::hook::hook;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::minwindef::TRUE;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

#[no_mangle]
pub extern "system" fn DllMain(module: HMODULE, reason: u32, _: u32) -> BOOL {
    return if reason == DLL_PROCESS_ATTACH {
        hook(module);
        TRUE
    } else {
        TRUE
    };
}

#[cfg(test)]
mod tests {
    use winapi::um::debugapi::OutputDebugStringW;

    #[test]
    fn it_works() {
        unsafe {
            OutputDebugStringW(encode("こんにちわ、世界！").as_ptr());
        }
    }

    fn encode(source: &str) -> Vec<u16> {
        source.encode_utf16().chain(Some(0)).collect()
    }
}
