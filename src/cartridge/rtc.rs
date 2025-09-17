use std::ffi::c_int;
use std::fs::File;
use std::os::raw::c_double;
use std::time;

pub struct RTC {
    pub filename: String,
    pub latch_enabled: bool,
    pub time_zero: c_double,
    pub time_lock: bool,
    pub sec_latch: u64,
    pub min_latch: u64,
    pub hour_latch: u64,
    pub day_latch_low: u64,
    pub day_latch_high: u64,
    pub day_carry: u64,
    pub halt: u64
}

impl RTC {
    pub fn new(filename: String) -> RTC {
        if filename == "" {
            return RTC {
                filename,
                latch_enabled: false,
                time_zero: 0.0,
                time_lock: false,
                sec_latch: 0,
                min_latch: 0,
                hour_latch: 0,
                day_latch_low: 0,
                day_latch_high: 0,
                day_carry: 0,
                halt: 0,
            }
        }

        let new_filename = filename + ".rtc";
        let mut time_zero = time::SystemTime::now();
        let time_lock = false;
        let day_carry = 0;
        let halt = 0;

        if !std::path::Path::new(&new_filename).exists() {
            panic!("RTC file does not exist");
        }
        else {
            let file = std::fs::File::open(new_filename).unwrap();

        }
    }

    fn load_state(&self, file: File, state_version: c_int, time_zero: &mut time::SystemTime) {
        if state_version <= 12 {
            self.time_zero = 0;
        }
    }
}