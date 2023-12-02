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