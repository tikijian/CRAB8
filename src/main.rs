
pub mod computer;
pub mod utils;

use crate::computer::Computer;

fn main() {
    let mut computer = Computer::new();
    let rom_data = utils::load_rom("IBM").unwrap();
    computer.load_rom(rom_data);

    // dbg!(computer.cpu.memory[0x503]);
    // for value in computer.cpu.memory[512..].iter() {
    //     print!("{}  ", *value);
    // }
}