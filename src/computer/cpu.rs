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
        // self.pc += 2; // TODO: check if it should be here
        self.opcode
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