
pub mod computer;
pub mod utils;

use crate::computer::Computer;

fn main() {
    let mut computer = Computer::new();
    computer.reset();

    let rom_data = utils::load_rom("IBM").unwrap();
    computer.load_rom(rom_data);

    // dbg!(computer.cpu.memory[0x503]);
    // for value in computer.cpu.memory[0x200..0x300].iter() {
    //     print!("{:#04x}  ", *value);
    // }
}