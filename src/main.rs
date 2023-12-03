
pub mod computer;

use crate::computer::Computer;

fn main() {
    let computer = Computer::new();
    dbg!(computer);
}