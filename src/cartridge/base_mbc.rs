use std::ffi::c_int;
use crate::cartridge::rtc::RTC;

pub struct BaseMBC {
    pub filename: String,
    pub game_title: String,
    pub rom_banks: Vec<u8>,
    pub ram_banks: Vec<u8>,
    pub cart_type: u8,
    pub battery_enabled: bool,
    pub rtc_enabled: bool,
    pub rtc: RTC,
    pub memory_module: u8,
    pub ram_bank_enabled: bool,
    pub external_ram_count: c_int,
    pub external_rom_count: c_int,
    pub ram_bank_initialized: bool,
    pub ram_bank_selected: u16,
    pub rom_bank_selected: u16,
    pub rom_bank_selected_low: u16,
    pub cgb_mode: bool,
}

impl BaseMBC {
    pub fn new(filename: String, game_title: String, rom_banks: Vec<u8>,
                external_ram_count: c_int, cart_type: u8, sram: u8,
                battery_enabled: bool, rtc_enabled: bool) -> BaseMBC
    {
        let new_filename = filename + ".ram";

        let mut rtc = None;
        if rtc_enabled {
            rtc = RTC
        }
    }
}


pub struct ROMOnly {
    pub mbc: BaseMBC,
}

impl ROMOnly {
    pub fn new() -> ROMOnly {

    }
    fn set_item(&mut self, address: u16, mut value: u16) {
        if 0x2000 <= address && address < 0x4000 {
            if value == 0 {
                value = 1;
            }
            self.mbc.rom_bank_selected = value & 0b1;
        }
        else if 0xA000 <= address && address < 0xC000 {
            self.mbc.ram_banks[self.mbc.ram_bank_selected..address-0xA000] = value;
        }
    }
}

