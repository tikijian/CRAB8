pub mod cpu;
pub mod display;

use core::fmt;
use cpu::CPU;
use display::Display;
use crate::utils::FONT;

pub const PROGRAM_START_ADDR: usize = 0x200;

pub struct Computer {
    pub cpu: CPU,
    // Display data
    pub display: Display,
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
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new(),

            display: Display::new(),
            keyboard: [0; 16],
            waiting_key: false,
            should_redraw: false,
            should_clear_screen: false,
            delay_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.display.reset();
        self.keyboard.fill(0);
        self.waiting_key = false;
        self.should_redraw = false;
        self.should_clear_screen = false;
        self.delay_timer = 0;
        
        self.load_font();
    }

    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        let end_addr = PROGRAM_START_ADDR + rom_data.len();
        self.cpu.memory[PROGRAM_START_ADDR..end_addr].copy_from_slice(rom_data.as_slice());
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.cpu.fetch_opcode();
        let op_key = opcode & 0xF000;

        match op_key {
            0 => {
                let op_key = opcode & 0x00FF;
                match op_key {
                    0xE0 => self.clear_screen(),
                    0xEE => self.cpu.return_from_subroutine(),
                    _ => ()
                }
            },
            0x1000 => self.cpu.jump_to_addr(),
            0x2000 => self.cpu.call_at_addr(),
            0x3000 => (),
            0x4000 => (),
            0x5000 => (),
            0x6000 => self.cpu.put_value_to_vx(),
            0x7000 => self.cpu.add_value_to_vx(),
            0x8000 => (),
            0x9000 => (),
            0xA000 => (),
            0xB000 => (),
            0xC000 => (),
            0xD000 => (),
            0xE000 => (),
            0xF000 => (),
            _ => panic!("Unknown opcode {:#04x}", opcode)
        };
    }

    fn load_font(&mut self) {
        self.cpu.memory[0..FONT.len()].copy_from_slice(&FONT);
    }

    fn clear_screen(&mut self) {
        self.display.reset();
        self.should_redraw = true;
    }
}

impl fmt::Debug for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Computer")
         .field("CPU", &self.cpu)
         .field("waiting_key", &self.waiting_key)
         .field("opcode", &self.cpu.opcode)
         .finish()
    }
}
