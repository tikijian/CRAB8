pub mod cpu;

use core::fmt;
use cpu::CPU;

pub struct Computer {
    pub cpu: CPU,
    // Display data
    pub display: [u8; 2048],
    // Keyboard with 16 keys
    pub keyboard: [u8; 16],
    // Wait-key flag
    pub waiting_key: bool,
    // Drawing flag - if true - SDL drawing occurs
    pub should_redraw: bool,
    // Clear screen flag - if true - SDL will clear screen
    pub shold_clear_screen: bool,
    // Delay timer
    pub delay_timer: u8,
    // Current opcode
    pub opcode: u16,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new(),

            display: [0; 2048],
            keyboard: [0; 16],
            waiting_key: false,
            should_redraw: false,
            shold_clear_screen: false,
            delay_timer: 0,
            opcode: 0,
        }
    }
}

impl fmt::Debug for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Computer")
         .field("CPU", &self.cpu)
         .field("waiting_key", &self.waiting_key)
         .field("opcode", &self.opcode)
         .finish()
    }
}