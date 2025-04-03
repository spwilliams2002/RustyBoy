use crate::motherboard::Motherboard;
use crate::motherboard::motherboard::Motherboard;

pub struct CPU {
    pub motherboard: Motherboard,

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

impl CPU {
    /// Creates a new CPU instance and initializes its state.
    pub fn new(mb: Motherboard) -> Self {
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

            motherboard: mb,

            bail: false,
            halted: false,
            stopped: false,
            is_stuck: false,
            cycles: 0,
            interrupts_flag: 0,
            interrupts_enabled: 0
        }
    }

    /// Resets the CPU to its initial state.
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
        self.memory.fill(0);
    }

    /// Executes a single instruction at the current program counter.
    pub fn step(&mut self) {
        let opcode = self.fetch_byte();
        self.execute_instruction(opcode);
    }

    /// Fetches the next byte from memory and increments the program counter.
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    /// Decodes and executes the instruction corresponding to the given opcode.
    fn execute_instruction(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.nop(),
            // Add more opcode implementations here...
            _ => panic!("Unknown opcode: {:#04x}", opcode),
        }
    }

    /// A placeholder for the NOP (No Operation) instruction.
    fn nop(&self) {
        // Does nothing
    }
}