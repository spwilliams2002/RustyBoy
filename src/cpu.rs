use crate::motherboard;
use crate::motherboard::Bus;

pub struct CPU {

    // Registers
    pub a: u8, // Accumulator
    pub f: u8, // Flags
    pub b: u8, // Register B
    pub c: u8, // Register C
    pub d: u8, // Register D
    pub e: u8, // Register E

    // Stack Pointer and Program Counter
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
    pub hl: u16,

    pub is_stuck: bool,
    pub interrupt_master_enable: bool,
    pub interrupt_queued: bool,
    pub halted: bool,
    pub stopped: bool,
    pub bail: bool,

    pub interrupts_flag: u8,
    pub interrupts_enabled: u8,
    pub interrupts_flag_register: u8,
    pub interrupts_enabled_register: u8,

    pub cycles: i64
}

// Flag bits
const FLAG_Z: u8 = 0b1000_000;
const FLAG_N: u8 = 0b0100_000;
const FLAG_H: u8 = 0b0010_000;
const FLAG_C: u8 = 0b0001_000;
const FLAG_MASK: u8 = 0xF0;

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            hl: 0,
            sp: 0xFFFE, // Default stack pointer initialization
            pc: 0x0100, // Default starting address for the Game Boy
            interrupts_flag_register: 0,
            interrupts_enabled_register: 0,
            interrupt_master_enable: false,
            interrupt_queued: false,

            bail: false,
            halted: false,
            stopped: false,
            is_stuck: false,
            cycles: 0,
            interrupts_flag: 0,
            interrupts_enabled: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.f = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.hl = 0;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> u32 {
        let opcode = self.fetch8(bus);
        self.execute_instruction(opcode, bus)
    }

    fn fetch8<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let byte = bus.read8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch16<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = self.fetch8(bus) as u16;
        let hi = self.fetch8(bus) as u16;
        (hi << 8) | lo
    }


    fn execute_instruction<B: Bus>(&mut self, opcode: u8, bus: &mut B) -> u32 {
        match opcode {
            // NOP
            0x00 => {self.nop(); 4},
            // LD BC, d16
            0x01 => {
                let data = self.fetch16(bus);
                self.set_bc(data);
                12
            }
            // LD (BC), A
            0x02 => {self.set_bc(self.a as u16); 8}
            // INC BC
            0x03 => {self.set_bc(self.bc().wrapping_add(1)); 8}
            // INC B
            0x04 => {self.b = self.inc8(self.b); 4}
            // DEC B
            0x05 => {self.b = self.dec8(self.b); 4}
            // LD B, d8
            0x06 => {
                let data = self.fetch8(bus);
                self.b = data;
                8
            }
            // RLCA
            0x07 => {self.rlca(); 4}
            // TODO: LD (a16), SP
            0x08 => {
                20
            }
            _ => panic!("Unknown opcode: {:#04x}", opcode),
        }
    }


    fn nop(&self) {
        // Does nothing
    }

    fn inc8(&mut self, r: u8) -> u8 {
        let old_val = r;
        let result = old_val.wrapping_add(1);

        let h = (old_val & 0x0F) == 0x0F;
        self.f = (self.f & FLAG_C)
            | if result == 0 { FLAG_Z } else { 0 }
            | if h { FLAG_H } else { 0 };

        result
    }
    fn dec8(&mut self, r: u8) -> u8 {
        let old_val = r;
        let result = old_val.wrapping_sub(1);

        let h = (old_val & 0x0F) == 0x00;
        self.f = (self.f & FLAG_C)
            | FLAG_N
            | if result == 0 { FLAG_Z } else { 0 }
            | if h { FLAG_H } else { 0 };

        result
    }

    fn bc(&self) -> u16 {((self.b as u16) << 8) | self.c as u16}
    fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    fn rlca(&mut self) {
        let carry = (self.a & 0x80) != 0;
        self.a = (self.a << 1) | if carry { 1 } else { 0 };
        self.f = if carry { FLAG_C } else { 0 };
    }
}