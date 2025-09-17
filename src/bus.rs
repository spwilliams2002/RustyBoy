/// Trait that allows the motherboard to pass values between its components
pub trait Bus {
    fn read8(&mut self, address: u16) -> u8;
    fn write8(&mut self, address: u16, value: u8);

    fn read16(&mut self, address: u16) -> u16 {
        let low = self.read8(address) as u16;
        let high = self.read8(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    fn write16(&mut self, address: u16, value: u16) {
        self.write8(address, (value & 0x00FF) as u8);
        self.write8(address.wrapping_add(1), (value >> 8) as u8);
    }
}

/// Mutable bus pointer that contains only necessary variables, avoids circular inheritance
pub struct BusMut<'a> {
    pub memory: &'a mut [u8],
}

impl<'a> Bus for BusMut<'a> {
    fn read8(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn write8(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}