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
    pub h: u8, // Register H
    pub l: u8, // Register L

    // Stack Pointer and Program Counter
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter

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
            h: 0,
            l: 0,
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
        self.h = 0;
        self.l = 0;
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
                let d16 = self.fetch16(bus);
                self.set_bc(d16);
                12
            }
            // LD BC, A
            0x02 => {self.set_bc(self.a as u16); 8}
            // INC BC
            0x03 => {self.set_bc(self.bc().wrapping_add(1)); 8}
            // INC B
            0x04 => {self.b = self.inc8(self.b); 4}
            // DEC B
            0x05 => {self.b = self.dec8(self.b); 4}
            // LD B, d8
            0x06 => {
                let d8 = self.fetch8(bus);
                self.b = d8;
                8
            }
            // RLCA
            0x07 => {self.rlca(); 4}
            // LD a16, SP
            0x08 => {
                let a16 = self.fetch16(bus);
                bus.write16(a16, self.sp);
                20
            }
            // ADD HL, BC
            0x09 => {
                let r = self.add16(self.hl(), self.bc());
                self.set_hl(r);
                20
            }
            // LD A, BC
            0x0A => {self.a = self.bc() as u8; 8}
            // DEC BC
            0x0B => {self.set_bc(self.bc().wrapping_sub(1)); 8}
            // INC C
            0x0C => {self.c = self.inc8(self.c); 4}
            // DEC C
            0x0D => {self.c = self.dec8(self.c); 4}
            // LD C, d8
            0x0E => {
                let d8 = self.fetch8(bus);
                self.c = d8;
                8
            }
            // RRCA
            0x0F => {self.rrca(); 4}
            // TODO: STOP
            0x10 => {4}
            // LD DE, d16
            0x11 => {
                let d16 = self.fetch16(bus);
                self.set_de(d16);
                12
            }
            // LD DE, A
            0x12 => {self.set_de(self.a as u16); 8}
            // INC DE
            0x13 => {self.set_de(self.de().wrapping_sub(1)); 8}
            // INC D
            0x14 => {self.d = self.inc8(self.d); 4}
            // DEC D
            0x15 => {self.d = self.dec8(self.d); 4}
            // LD D, d8
            0x16 => {
                let d8 = self.fetch8(bus);
                self.d = d8;
                8
            }
            // 
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

    fn add16(&mut self, x: u16, y: u16) -> u16 {
        let mut result = x + y;
        let mut flag = 0b0000_000;
        flag += if ((self.hl() & 0xFFF) + (self.bc() & 0xFFF)) > 0xFFF { FLAG_H } else { 0 };
        flag += if result > 0xFFFF { FLAG_C } else { 0 };
        self.f &= 0b1000_000;
        self.f |= flag;
        result |= 0xFFFF;
        result
    }

    fn bc(&self) -> u16 {((self.b as u16) << 8) | self.c as u16}
    fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    fn de(&self) -> u16 {((self.d as u16) << 8) | self.e as u16}
    fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    fn hl(&self) -> u16 {((self.h as u16) << 8) | self.l as u16}
    fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    fn rlca(&mut self) {
        let mut new_a = (self.a << 1) + (self.a >> 7);
        let mut flag = 0b0000_000;
        flag += if new_a > 0xFF { FLAG_C } else { 0 };
        self.f &= 0b1000_000;
        self.f |= flag;
        new_a &= 0xFF;
        self.a = new_a;
    }
    fn rrca(&mut self) {
        let mut new_a = (self.a >> 1) + ((self.a & 1) << 7) + ((self.a & 1) << 8);
        let mut flag = 0b0000_000;
        flag += if new_a > 0xFF { FLAG_C } else { 0 };
        self.f &= 0b1000_000;
        self.f |= flag;
        new_a &= 0xFF;
        self.a = new_a;
    }
}


// Tests
#[test]
fn inc_flags() {
    let mut cpu = CPU::new();
    cpu.f = FLAG_C;           // C should be preserved
    cpu.b = 0x0F; cpu.b = cpu.inc8(cpu.b);
    assert_eq!(cpu.b, 0x10);
    assert_eq!(cpu.f, FLAG_C | FLAG_H); // H=1, Z=0, N=0

    cpu.f = 0; cpu.b = 0xFF; cpu.b = cpu.inc8(cpu.b);
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.f, FLAG_Z | FLAG_H); // Z=1, H=1, C unchanged(0), N=0
}

#[test]
fn dec_flags() {
    let mut cpu = CPU::new();
    cpu.f = FLAG_C;           // C should be preserved
    cpu.b = 0x10; cpu.b = cpu.dec8(cpu.b);
    assert_eq!(cpu.b, 0x0F);
    assert_eq!(cpu.f, FLAG_C | FLAG_N | FLAG_H); // N=1, H=1

    cpu.f = 0; cpu.b = 0x01; cpu.b = cpu.dec8(cpu.b);
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.f, FLAG_N | FLAG_Z); // H=0, C unchanged(0)
}