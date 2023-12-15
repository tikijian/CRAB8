use std::fmt;

#[derive(Debug, Clone)]
pub struct Opcode(u16);

impl Opcode {
    pub fn new(value: u16) -> Opcode {
        Opcode(value)
    }

    pub fn from(upper_byte: u8, lower_byte: u8) -> Opcode {
        let opcode: u16 = ((upper_byte as u16) << 8u8) | lower_byte as u16;
        Opcode(opcode)
    }

    pub fn get_x(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }
    
    pub fn get_y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }

    pub fn get_z(&self) -> u8 {
        (self.0 & 0x00F) as u8
    }

    pub fn get_nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }

    pub fn get_nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl fmt::LowerHex for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        fmt::LowerHex::fmt(&val, f)
    }
}