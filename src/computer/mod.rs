pub mod cpu;
pub mod display;
pub mod opcode;

use core::fmt;
use cpu::CPU;
use display::Display;
use crate::utils::FONT;

use self::opcode::Opcode;

pub const PROGRAM_START_ADDR: usize = 0x200;

pub struct Computer {
    pub cpu: CPU,
    // Display data
    pub display: Display,
    // Keyboard with 16 keys
    pub keyboard: [bool; 16], // TODO: Keyboard type
    // Wait-key flag
    pub waiting_key: bool,
    // Drawing flag - if true - SDL drawing occurs
    pub should_redraw: bool,
    // Clear screen flag - if true - SDL will clear screen
    pub should_clear_screen: bool,
    // Delay timer
    pub delay_timer: u8,
    // Sound timer
    pub sound_timer: u8,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            cpu: CPU::new(),
            display: Display::new(),

            keyboard: [false; 16],
            waiting_key: false,
            should_redraw: false,
            should_clear_screen: false,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.display.reset();
        self.keyboard.fill(false);
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
        let op_key = opcode.value() & 0xF000;

        match op_key {
            0 => {
                let op_key = opcode.get_nn();
                match op_key {
                    0xE0 => self.clear_screen(),
                    0xEE => self.cpu.return_from_subroutine(),
                    _ => self.unknow_opcode_error(opcode)
                }
            },
            0x1000 => self.cpu.jump_to_addr(),
            0x2000 => self.cpu.call_at_addr(),
            0x3000 => self.cpu.skip_3xkk(),
            0x4000 => self.cpu.skip_4xkk(),
            0x5000 => self.cpu.skip_5xy(),
            0x6000 => self.cpu.put_value_to_vx(),
            0x7000 => self.cpu.add_value_to_vx(),
            0x8000 => {
                let op_key = opcode.get_z();
                match op_key {
                    0 => self.cpu.vy_to_vx(),
                    0x1 => self.cpu.vx_or_vy(),
                    0x2 => self.cpu.vx_and_vy(),
                    0x3 => self.cpu.vx_xor_vy(),
                    0x4 => self.cpu.vx_add_vy(),
                    0x5 => self.cpu.vx_sub_vy(),
                    0x6 => self.cpu.vx_shr(),
                    0x7 => self.cpu.vy_sub_vx(),
                    0xE => self.cpu.vx_shl(),
                    _ => self.unknow_opcode_error(opcode)
                }
            },
            0x9000 => self.cpu.skip_9xy(),
            0xA000 => self.cpu.set_i_reg(),
            0xB000 => self.cpu.jump_to_addr_offset(),
            0xC000 => self.cpu.add_random_to_vx(),
            0xD000 => self.draw_sprite(),
            0xE000 => {
                let op_key = opcode.get_nn();
                match op_key {
                    0x9E => self.cpu.skip_on_keydown(&self.keyboard),
                    0xA1 => self.cpu.skip_on_keyup(&self.keyboard), 
                    _ => self.unknow_opcode_error(opcode)
                }
            },
            0xF000 => {
                let op_key = opcode.get_nn();
                match op_key {
                    0x07 => self.cpu.set_vx(self.delay_timer),
                    0x0A => {
                        // TODO: stop execution on keypress await
                        self.waiting_key = true;
                    },
                    0x15 => self.delay_timer = self.cpu.get_vx(),
                    0x18 => self.sound_timer = self.cpu.get_vx(),
                    0x1E => self.cpu.add_vx_to_i(),
                    0x29 => self.cpu.set_font_char_addr(),
                    0x33 => self.cpu.vx_decimal_to_ireg(),
                    0x55 => self.cpu.store_regs_in_memory(),
                    0x65 => self.cpu.store_memory_in_regs(),
                    _ => self.unknow_opcode_error(opcode)
                }
            },
            _ => self.unknow_opcode_error(opcode)
        };
    }

    fn load_font(&mut self) {
        self.cpu.memory[0..FONT.len()].copy_from_slice(&FONT);
    }

    fn draw_sprite(&mut self) {
        self.cpu.draw_sprite(&mut self.display);
        self.should_redraw = true;
    }

    fn clear_screen(&mut self) {
        self.display.reset();
        self.should_clear_screen = true;
    }

    fn unknow_opcode_error(&self, opcode: Opcode) -> ! {
        panic!("Unknown opcode {:#04x}", opcode.value())
    }

    fn register_keypress(&mut self, key_index: u8) {
        self.keyboard[key_index as usize] = true;
        self.waiting_key = false;
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
