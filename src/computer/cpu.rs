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

        self.opcode = opcode;
        self.pc += 2; // TODO: check if it should be here
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
        let reg_id = (self.opcode & 0x0F00) as usize;
        let current_value = self.regs[reg_id];

        self.set_vx(current_value + value);
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

    fn get_vx(&self) -> u8 {
        let reg_id = (self.opcode & 0x0F00) as usize;
        self.regs[reg_id]
    }

    fn get_vy(&self) -> u8 {
        let reg_id = (self.opcode & 0x00F0) as usize;
        self.regs[reg_id]
    }

    fn set_vx(&mut self, value: u8) {
        let reg_id = (self.opcode & 0x0F00) as usize;
        self.regs[reg_id] = value;
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