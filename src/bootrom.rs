mod bootrom {
    pub struct BootROM {
        pub sp: u16,
        pub pc: u16,
        pub vram: [u8; 0x2000],

    }

}
