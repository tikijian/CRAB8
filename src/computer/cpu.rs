
use crate::computer::display::Display;
use crate::computer::display::WIDTH as DISPLAY_WIDTH;
// use crate::computer::display::HEIGHT as DISPLAY_HEIGHT;

use core::fmt;

pub struct CPU {
    // 4KB RAM
    pub memory: [u8; 4096],
    // 16 general 8-bit registers
    pub regs: [u8; 16],
    // 16-bit index register
    pub i_reg: u16,
    // Service register
    pub vf: bool,
    // 16-bit Program Counter
    pub pc: usize,
    // 8-bit Stack pointer
    pub sp: usize,
    // Stack
    pub stack: [u16; 16],
    pub opcode: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            regs: [0; 16],
            i_reg: 0,
            vf: false,
            pc: 0,
            sp: 0,
            stack: [0; 16],
            opcode: 0
        }
    }

    pub fn reset(&mut self) {
        self.sp = 0;
        self.i_reg = 0;
        self.sp = 0;
        self.regs.fill(0);
        self.stack.fill(0);
        self.memory.fill(0);
        self.pc = super::PROGRAM_START_ADDR;
    }

    pub fn fetch_opcode(&mut self) -> u16 {
        let mut opcode: u16 = self.memory[self.pc].into();
        opcode <<= 8u8;
        opcode |= self.memory[self.pc + 1] as u16;

        self.pc += 2; // TODO: check if it should be here
        self.opcode = opcode;
        println!("OPCODE: {:#04x}", self.opcode);
        self.opcode
    }

    // 00E0 
    pub fn return_from_subroutine(&mut self) {
        self.pc = self.stack[self.sp] as usize;
        self.sp -= 1;
    }

    // 1nnn
    pub fn jump_to_addr(&mut self) {
        self.pc = (self.opcode & 0x0FFF).into();
    }

    // 2nnn
    pub fn call_at_addr(&mut self) {
        self.sp += 1;
        self.stack[self.sp] = self.pc as u16;
        self.jump_to_addr()
    }

    pub fn skip_3xkk(&mut self) {
        let value: u8 = (self.opcode & 0x00FF) as u8;
        if self.get_vx() == value {
            self.pc += 2;
        }
    }
    
    pub fn skip_4xkk(&mut self) {
        let value: u8 = (self.opcode & 0x00FF) as u8;
        if self.get_vx() != value {
            self.pc += 2;
        }
    }
    
    pub fn skip_5xy(&mut self) {
        if self.get_vx() == self.get_vy() {
            self.pc += 2;
        }
    }

    pub fn skip_9xy(&mut self) {
        if self.get_vx() != self.get_vy() {
            self.pc += 2;
        }
    }

    // 6xkk
    pub fn put_value_to_vx(&mut self) {
        let value: u8 = (self.opcode & 0x00FF) as u8;
        self.set_vx(value);
    }
    
    // 7xkk
    pub fn add_value_to_vx(&mut self) {
        let value: u8 = (self.opcode & 0x00FF) as u8;
        println!("   ADD {:#04x} + {:#04x}", self.get_vx(), value);
        self.set_vx(self.get_vx() + value);
    }

    // Annn
    pub fn set_i_reg(&mut self) {
        self.i_reg = self.opcode & 0x0FFF;
    } 

    // Bnnn
    pub fn jump_to_addr_offset(&mut self) {
        let addr: u16 = (self.opcode & 0x0FFF) + self.regs[0] as u16;
        self.pc = addr.into();
    }

    // Dxyn
    pub fn draw_sprite(&mut self, display: &mut Display) {
        let x = self.get_vx();// & (DISPLAY_WIDTH - 1);
        let y = self.get_vy();// & (DISPLAY_HEIGHT - 1);
        dbg!(x);
        dbg!(y);
        let height: u8 = (self.opcode & 0x000F) as u8;
        self.regs[0xF] = 0;
        
        for y_line in 0..height {
            let pixel = self.memory[(self.i_reg + y_line as u16) as usize];

            // for each bit out of 8 bytes...
            for x_line in 0..8u16 {
                if (pixel & (0x80 >> x_line)) != 0 {
                    let position = (x as u16 + x_line as u16 + ((y + y_line) as u16 * DISPLAY_WIDTH as u16)) as usize;
                    
                    if display.memory[position] == 1 {
                        self.regs[0xF] = 1;
                    }
                    
                    display.memory[position] ^= 1;
                }
            }
        }
    }

    fn get_vx(&self) -> u8 {
        let reg_id = ((self.opcode & 0x0F00) >> 8) as usize;
        self.regs[reg_id]
    }

    fn set_vx(&mut self, value: u8) {
        let reg_id = ((self.opcode & 0x0F00) >> 8) as usize;
        println!("   SET V{} - {:#04x}", reg_id, value);
        self.regs[reg_id] = value;
    }
    
    fn get_vy(&self) -> u8 {
        let reg_id = ((self.opcode & 0x00F0) >> 4) as usize;
        self.regs[reg_id]
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
         .field("regs", &self.regs)
         .field("i_reg", &self.i_reg)
         .field("vf", &self.vf)
         .field("pc", &self.pc)
         .field("sp", &self.sp)
         .field("stack", &self.stack)
         .finish()
    }
}