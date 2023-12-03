use core::fmt;

pub struct CPU {
    // 4KB RAM
    memory: [u8; 4096],
    // 16 general 8-bit registers
    regs: [u8; 16],
    // 16-bit index register
    i_reg: u16,
    // Service register
    vf: bool,
    // 16-bit Program Counter
    pc: u16,
    // 8-bit Stack pointer
    sp: u8,
    // Stack
    stack: [u16; 16],
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
        }
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