unsafe fn tamper_bytes(addr: u32, data: &[u8]) -> Vec<u8> {
    let mut old = Vec::new();
    for i in 0..data.len() {
        let ptr = (addr + i as u32) as *mut u8;
        old.push(*ptr);
        *ptr = data[i];
    }
    old
}

pub struct Tamperer {
    address: u32,
    original_operators: Vec<u8>,
}

impl Tamperer {
    pub unsafe fn near_jmp_operator(addr: u32, target: u32) -> Self {
        let operator = *(addr as *const u8);
        let mut data = (((target as i64) - (addr + 5) as i64) as i32)
            .to_le_bytes()
            .to_vec();
        data.insert(0, operator);
        Self::bytes(addr, &data)
    }

    pub unsafe fn byte(addr: u32, data: u8) -> Self {
        Self::bytes(addr, &[data])
    }

    pub unsafe fn bytes(addr: u32, data: &[u8]) -> Self {
        Self {
            address: addr,
            original_operators: tamper_bytes(addr, data),
        }
    }

    pub fn jmp_target_addr(&self) -> u32 {
        let mut dst = [0u8; 4];
        dst.clone_from_slice(&self.original_operators[1..5]);
        let reference = i32::from_le_bytes(dst);
        (reference + (self.address as i32 + 5)) as u32
    }
}

impl Drop for Tamperer {
    fn drop(&mut self) {
        unsafe {
            tamper_bytes(self.address, &self.original_operators);
        }
    }
}
