pub(crate) use crate::bus::Bus;
use crate::bus::BusMut;
use crate::cpu::CPU;

pub struct Motherboard {
    pub cpu: CPU,
    memory: Vec<u8>,
}

impl Motherboard {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: vec![0; 0x10000],
        }
    }

    pub fn run_frame(&mut self) {
        let mut cycles = 0;
        while cycles < 70_224 {
            let c = {
                let (cpu, mem) = (&mut self.cpu, &mut self.memory);
                let mut bus = BusMut { memory: mem };
                cpu.step(&mut bus)
            };
            cycles += c as i64;
        }
    }
}

impl Bus for Motherboard {
    fn read8(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
