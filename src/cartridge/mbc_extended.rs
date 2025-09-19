use std::ffi::c_int;
use crate::cartridge::base_mbc::BaseMBC;

pub struct MBC1 {
    pub base_mbc: BaseMBC,
    pub bank_select_register1: u8,
    pub bank_select_register2: u8,
}

pub struct MBC2 {
    pub base_mbc: BaseMBC,
}

pub struct MBC3 {
    pub base_mbc: BaseMBC,
}

pub struct MBC5 {
    pub base_mbc: BaseMBC,
}

impl MBC1 {
    pub fn new(&mut self, filename: String, rom_banks: Vec<u8>,
               external_ram_count: c_int, cart_type: u8, sram: u8,
               battery_enabled: bool, rtc_enabled: bool) -> Self {
        Self {
            base_mbc: BaseMBC::new(filename, rom_banks, external_ram_count, cart_type,
            sram, battery_enabled, rtc_enabled),
            bank_select_register1: 1,
            bank_select_register2: 0
        }
    }

    pub fn set_item(&mut self, address: u16, mut value: u8) {
        if (0x0000 <= address) && (address < 0x2000) {
            self.base_mbc.ram_bank_enabled = (value & 0b00001111) == 0b1010
        }
        else if (0x2000 <= address) && (address < 0x4000) {
            value &= 0b00011111;
            if value == 0 { value = 1; }
            self.bank_select_register1 = value
        }
        else if (0x4000 <= address) && (address < 0x6000) {
            self.bank_select_register2 = value & 0b11;
        }
        else if (0x6000 <= address) && (address < 0x8000) {
            self.base_mbc.memory_model = value & 0b1
        }
        else if (0xA000 <= address) && (address < 0xC000) {
            if self.base_mbc.ram_bank_enabled {
                self.base_mbc.ram_bank_selected =
                    if self.base_mbc.memory_model == 1 { self.bank_select_register2 as u16 }
                    else { 0 };
                let range_start = (self.base_mbc.ram_bank_selected % self.base_mbc.external_ram_count as u16) as usize;
                let range_end = (address - 0xA000) as usize;
                for v in &mut self.base_mbc.ram_banks[range_start..range_end] {
                    *v = value;
                }
            }
        }
        else {
            panic!["Invalid writing address: {}", address];
        }

        if self.base_mbc.memory_model == 1 {
            self.base_mbc.rom_bank_selected_low = ((self.bank_select_register2 << 5) & self.base_mbc.external_rom_count as u8) as u16;
        }
        else {
            self.base_mbc.rom_bank_selected_low = 0;
        }

        self.base_mbc.rom_bank_selected = (((self.bank_select_register2 << 5) | self.bank_select_register1) % self.base_mbc.external_rom_count as u8) as u16;
    }

    pub fn get_item(&mut self, address: u16) -> u8 {
        if (0xA000 <= address) && (address < 0xC000) {
            if !self.base_mbc.ram_bank_initialized {
                panic!["RAM banks not initialized: {}", address];
            }
            if !self.base_mbc.ram_bank_enabled {
                return 0xFF
            }
            if self.base_mbc.memory_model == 1 {
                self.base_mbc.ram_bank_selected = (self.bank_select_register2 % self.base_mbc.external_ram_count as u8) as u16;
            }
            else {
                self.base_mbc.ram_bank_selected = 0;
            }
            self.base_mbc.ram_banks[self.base_mbc.ram_bank_selected as usize..(address-0xA000) as usize].iter().sum::<u8>()
        }
        else {
            panic!["Invalid reading address: {}", address];
        }
    }
}

impl MBC2 {
    pub fn new(&mut self, filename: String, rom_banks: Vec<u8>,
               external_ram_count: c_int, cart_type: u8, sram: u8,
               battery_enabled: bool, rtc_enabled: bool) -> Self {
        Self {
            base_mbc: BaseMBC::new(filename, rom_banks, external_ram_count, cart_type,
                                   sram, battery_enabled, rtc_enabled),
        }
    }

    pub fn set_item(&mut self, address: u16, mut value: u8) {
        if (0x0000 <= address) && (address < 0x4000) {
            value &= 0b00001111;
            if (address & 0x100) == 0 {
                self.base_mbc.ram_bank_enabled = value == 0b00001010;
            }
            else {
                if value == 0 { value = 1; }
            }
        }
        else if (0xA000 <= address) && (address < 0xC000) {
            if self.base_mbc.ram_bank_enabled {
                let range_start = 0;
                let range_end = (address % 512) as usize;
                for v in &mut self.base_mbc.ram_banks[range_start..range_end] {
                    *v = value | 0b11110000;
                }
            }
        }
        else {
            panic!["Invalid writing address: {}, value: {}", address, value];
        }
    }

    pub fn get_item(&mut self, address: u16) -> u8 {
        if (0x0000 <= address) && (address < 0x4000) {
            return self.base_mbc.ram_banks[0..address as usize].iter().sum::<u8>();
        }
        else if (0x4000 <= address) && (address < 0x8000) {
            return self.base_mbc.ram_banks[self.base_mbc.ram_bank_selected as usize..(address - 0x4000) as usize].iter().sum::<u8>();
        }
        else if (0xA000 <= address) && (address < 0xC000) {
            if !self.base_mbc.ram_bank_initialized {
                panic!["RAM banks not initialized: {}", address];
            }
            if !self.base_mbc.ram_bank_enabled {
                return 0xFF;
            }
            else {
                return self.base_mbc.ram_banks[0..(address % 512) as usize].iter().sum::<u8>();
            }
        }
        else {
            panic!["Invalid reading address: {}", address];
        }
    }
}

impl MBC3 {
    pub fn new(&mut self, filename: String, rom_banks: Vec<u8>,
               external_ram_count: c_int, cart_type: u8, sram: u8,
               battery_enabled: bool, rtc_enabled: bool) -> Self {
        Self {
            base_mbc: BaseMBC::new(filename, rom_banks, external_ram_count, cart_type,
                                   sram, battery_enabled, rtc_enabled),
        }
    }
    pub fn set_item(&mut self, address: u16, mut value: u8) {
        if (0x0000 <= address) && (address < 0x2000) {
            if (value & 0b00001111) == 0b1010 {
                self.base_mbc.ram_bank_enabled = true;
            }
            else if value == 0 {
                self.base_mbc.ram_bank_enabled = false;
            }
            else {
                self.base_mbc.ram_bank_enabled = false;
                print!["Unexpected command for MBC3 at address: {}", address];
            }
        }
        else if (0x2000 <= address) && (address < 0x4000) {
            value &= 0b01111111;
            if value == 0 {
                value = 1;
            }
            self.base_mbc.rom_bank_selected = (value % self.base_mbc.external_rom_count as u8) as u16;
        }
        else if (0x4000 <= address) && (address < 0x6000) {
            if (0x08 <= value) && (value < 0x0C) {
                self.base_mbc.ram_bank_selected = value as u16;
            } else {
                self.base_mbc.ram_bank_selected = (value % self.base_mbc.external_ram_count as u8) as u16;
            }
        }
        else if (0x6000 <= address) && (address < 0x8000) {
            if self.base_mbc.rtc_enabled {
                self.base_mbc.rtc.write_command(value);
            }
        }
        else if (0xA000 <= address) && (address < 0xC000) {
            if self.base_mbc.ram_bank_enabled {
                if self.base_mbc.ram_bank_selected <= 0x03 {
                    for v in &mut self.base_mbc.ram_banks[self.base_mbc.ram_bank_selected as usize..(address-0xA000) as usize] {
                        *v = value;
                    }
                }
                else if (0x08 <= self.base_mbc.ram_bank_selected) && (self.base_mbc.ram_bank_selected <= 0x0C) {
                    self.base_mbc.rtc.set_register(self.base_mbc.ram_bank_selected, value);
                }
                else {
                    panic!["Invalid RAM bank selected: {}", self.base_mbc.ram_bank_selected];
                }
            }
            else {
                panic!["Invalid writing address: {}", address];
            }
        }
    }
}

impl MBC5 {
    pub fn new(&mut self, filename: String, rom_banks: Vec<u8>,
               external_ram_count: c_int, cart_type: u8, sram: u8,
               battery_enabled: bool, rtc_enabled: bool) -> Self {
        Self {
            base_mbc: BaseMBC::new(filename, rom_banks, external_ram_count, cart_type,
                                   sram, battery_enabled, rtc_enabled),
        }
    }

    pub fn set_item(&mut self, address: u16, mut value: u8) {
        if (0x0000 <= address) && (address < 0x2000) {
            self.base_mbc.ram_bank_enabled = value == 0b00001010;
        }
        else if (0x2000 <= address) && (address < 0x3000) {
            self.base_mbc.rom_bank_selected = ((self.base_mbc.rom_bank_selected & 0b100000000) |
                value as u16) % self.base_mbc.external_rom_count as u16;
        }
        else if (0x3000 <= address) && (address < 0x4000) {
            self.base_mbc.rom_bank_selected = ((((value & 0x1) << 8) |
                (self.base_mbc.rom_bank_selected as u8 & 0xFF)) %
                self.base_mbc.external_rom_count as u8) as u16
        }
        else if (0x4000 <= address) && (address < 0x6000) {
            self.base_mbc.ram_bank_selected = ((value & 0xF) % self.base_mbc.external_ram_count as u8) as u16;
        }
        else if (0xA000 <= address) && (address < 0xC000) {
            if self.base_mbc.ram_bank_enabled {
                for v in &mut self.base_mbc.ram_banks[self.base_mbc.ram_bank_selected as usize..(address-0xA000) as usize] {
                    *v = value;
                }
            }
        }
        else {
            print!["Unexpected write to {}, {}", address, value];
        }
    }
}
