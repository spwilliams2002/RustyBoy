mod cartridge;
mod util;
mod cpu;
mod motherboard;
mod system;
mod bus;
// use cartridge::cartridge::Cartridge;

extern crate std;

use crate::motherboard::Motherboard;
use crate::cartridge::cartridge::Cartridge;

fn main() {
    let motherboard = Motherboard::new();

}
