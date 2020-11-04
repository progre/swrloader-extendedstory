#[cfg(not(debug_assertions))]
pub fn d(_: &str) {}

#[cfg(debug_assertions)]
pub fn d(msg: &str) {
    use winapi::um::debugapi::OutputDebugStringW;

    let vec = encode(msg);
    unsafe {
        OutputDebugStringW(vec.as_ptr());
    }

    fn encode(source: &str) -> Vec<u16> {
        source.encode_utf16().chain(Some(0)).collect()
    }
}
