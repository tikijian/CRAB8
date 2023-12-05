pub mod cpu;

use core::fmt;
use cpu::CPU;
use crate::utils::FONT;

pub const PROGRAM_START_ADDR: usize = 0x200;

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
    pub should_clear_screen: bool,
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
            should_clear_screen: false,
            delay_timer: 0,
            opcode: 0,
        }
    }

    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        let end_addr = PROGRAM_START_ADDR + rom_data.len();
        self.cpu.memory[PROGRAM_START_ADDR..end_addr].copy_from_slice(rom_data.as_slice());
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.display.fill(0);
        self.keyboard.fill(0);
        self.waiting_key = false;
        self.should_redraw = false;
        self.should_clear_screen = false;
        self.delay_timer = 0;
        
        self.load_font();
    }

    fn load_font(&mut self) {
        self.cpu.memory[0..FONT.len()].copy_from_slice(&FONT);
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
