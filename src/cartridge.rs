use std::collections::HashMap;

mod cartridge {
    use std::collections::HashMap;
    use std::ptr::null;

    pub struct Cartridge {
        pub rom: ROM,
        pub ram: RAM,
        pub mbc: MBC,
        pub cartridge_table: HashMap<u8, std::any>,
        pub external_ram_table: HashMap<u8, u8>,
    }
    impl Cartridge {
        pub fn new(rom: ROM, ram: RAM, mbc: MBC) -> Cartridge {
            let mut cartridge_table = HashMap::new();
            cartridge_table.insert(0x00, (false, false, false));
            cartridge_table.insert(0x01, (false, false, false));
            cartridge_table.insert(0x02, (true, false, false));
            cartridge_table.insert(0x03, (true, true, false));
            cartridge_table.insert(0x05, (false, false, false));
            cartridge_table.insert(0x06, (false, true, false));
            cartridge_table.insert(0x08, (true, false, false));
            cartridge_table.insert(0x09, (true, true, false));
            cartridge_table.insert(0x0F, (false, true, true));
            cartridge_table.insert(0x10, (true, true, true));
            cartridge_table.insert(0x11, (false, false, false));
            cartridge_table.insert(0x12, (true, false, false));
            cartridge_table.insert(0x13, (true, true, false));
            cartridge_table.insert(0x19, (false, false, false));
            cartridge_table.insert(0x1A, (true, false, false));
            cartridge_table.insert(0x1B, (true, true, false));
            cartridge_table.insert(0x1C, (false, false, false));
            cartridge_table.insert(0x1D, (true, false, false));
            cartridge_table.insert(0x1E, (true, true, false));

            let mut external_ram_table = HashMap::new();
            external_ram_table.insert(0x00, 1);
            external_ram_table.insert(0x01, 1);
            external_ram_table.insert(0x02, 1);
            external_ram_table.insert(0x03, 4);
            external_ram_table.insert(0x04, 16);
            external_ram_table.insert(0x05, 8);

            Cartridge { rom, ram, mbc, cartridge_table, external_ram_table }
        }

        fn load_cartridge(&self, filename: &str) {
            let rom_banks = load_rom(filename);
            if !validate_cartridge(rom_banks) {
                panic!("Invalid cartridge");
            }

            let external_ram_count = *self.external_ram_table.get(&rom_banks[0x0149]).unwrap_or(&0);
            let cart_type = rom_banks[0x0147];
            let cart_info = self.cartridge_table.get(&cart_type).unwrap_or(&(false, false, false));
            if cart_info == null() {
                panic!("Invalid cartridge type");
            }
            // car_line =
        }


    }

    fn validate_cartridge(rom_banks: &Vec<u8>) -> bool {
        let mut x = 0;
        for i in 0x134..0x14D {
            x = x - rom_banks[i] - 1;
            x &= 0xFF;
        }
        rom_banks[0x14D] == x
    }

    fn load_rom(filename: &str) -> &Vec<u8> {
        let rom_data = std::fs::read(filename).unwrap().to_vec();
        if Vec::len(&rom_data) == 0x0 {
            panic!("File is empty");
        }
        let bank_size = 16 * 1024;
        if Vec::len(&rom_data) % bank_size != 0 {
            panic!("File is not a multiple of 16kb");
        }
        &rom_data
    }

    pub struct MBC {
        pub type_id: u8,
        pub rom: ROM,
        pub ram: RAM,
        pub rom_bank: u8,
        pub ram_bank: u8,
        pub mode: u8,
        pub rtc_enabled: bool,
        pub rtc_register: u8,
        pub rtc_data: u8,
        pub rtc_latch: u8,
        pub rtc_clock_divider: u8,
    }
    impl MBC {}


    pub struct ROM {
        pub data: Vec<u8>,
        pub mbc: MBC,
        pub ram: Vec<u8>,
        pub rom_bank: u8,
        pub ram_bank: u8,
        pub mode: u8,
        pub rtc_enabled: bool,
        pub rtc_register: u8,
        pub rtc_data: u8,
        pub rtc_latch: u8,
        pub rtc_clock_divider: u8,
        pub rtc_clock_counter: u8,
    }
    impl ROM {}
    pub struct RAM {
        pub data: Vec<u8>,
        pub mode: u8,
        pub bank: u8,
        pub enable: bool,
        pub rtc_enabled: bool,
        pub rtc_register: u8,
        pub rtc_data: u8,
        pub rtc_latch: u8,
        pub rtc_clock_divider: u8,
        pub rtc_clock_counter: u8,
    }
    impl RAM {}
}


