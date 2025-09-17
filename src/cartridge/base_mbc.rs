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
    pub memory_model: u8,
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
    pub fn new(&mut self, filename: String, rom_banks: Vec<u8>,
                external_ram_count: c_int, cart_type: u8, sram: u8,
                battery_enabled: bool, rtc_enabled: bool) -> Self
    {
            let new_filename = filename + ".ram";

            let mut rtc = RTC::new(String::from(""));
            if rtc_enabled {
                rtc = RTC::new(new_filename.clone());
            }

            let mut ram_bank_initialized = false;
            let external_rom_count = rom_banks.iter().count() as c_int;
            let ram_banks = BaseMBC::init_ram_banks(external_ram_count, &mut ram_bank_initialized);

            let cgb_mode = rom_banks[0..0x0143].iter().count() > 7;

            Self {
                filename: new_filename,
                game_title: BaseMBC::get_game_title(rom_banks.clone(), cgb_mode),
                rom_banks,
                ram_banks: vec![],
                cart_type,
                battery_enabled,
                rtc_enabled,
                rtc,
                memory_model: 0,
                ram_bank_enabled: false,
                external_ram_count,
                external_rom_count,
                ram_bank_initialized,
                ram_bank_selected: 0,
                rom_bank_selected: 1,
                rom_bank_selected_low: 0,
                cgb_mode,
            }
    }

    fn init_ram_banks(n: c_int, ram_bank_initialized: &mut bool) -> Vec<u8> {
        ram_bank_initialized = true;
        let ram_banks = vec![0; n as usize];
        ram_banks
    }

    fn get_game_title(rom_banks: Vec<u8>, cgb_mode: bool) -> String {
        let mut end = 0x0143;
        if cgb_mode {
            end = 0x0142;
        }

        let mut title = "".parse::<String>().unwrap();
        for x in 0x0134..end {
            title = title + &*(rom_banks[x].to_string());
        }
        title
    }
}


pub struct ROMOnly {
    pub mbc: BaseMBC,
}

// impl ROMOnly {
//     pub fn new() -> Self {
//         Self {
//         mbc: BaseMBC::new()
//             }
//     }
//     fn set_item(&mut self, address: u16, mut value: u16) {
//         if 0x2000 <= address && address < 0x4000 {
//             if value == 0 {
//                 value = 1;
//             }
//             self.mbc.rom_bank_selected = value & 0b1;
//         }
//         else if 0xA000 <= address && address < 0xC000 {
//             self.mbc.ram_banks[self.mbc.ram_bank_selected..address-0xA000] = value;
//         }
//     }
// }

