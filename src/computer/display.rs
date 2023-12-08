pub const WIDTH: u8 = 63;
pub const HEIGHT: u8 = 31;

pub struct Display {
    pub memory: [u8; 2048],
}

impl Display {
    pub fn new() -> Display {
        Display { memory: [0; 2048] }
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
    }
}