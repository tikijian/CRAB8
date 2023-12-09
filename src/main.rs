
pub mod computer;
pub mod utils;

use crate::computer::Computer;

fn main() {
    let mut computer = Computer::new();
    computer.reset();

    let rom_data = utils::load_rom("IBM").unwrap();
    computer.load_rom(rom_data);

    // for value in computer.cpu.memory[0x200..0x210].iter() {
    //     print!("{:#04x}  ", *value);
    // }
    // println!("opcodes: ");
    // println!("{:#04x}", computer.cpu.fetch_opcode());
    // println!("{:#04x}", computer.cpu.fetch_opcode());
    // println!("{:#04x}", computer.cpu.fetch_opcode());
    // println!("{:#04x}", computer.cpu.fetch_opcode());
    print!("{:#04x}", 0xa22a & 0xF000);
}