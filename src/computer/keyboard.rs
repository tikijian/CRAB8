extern crate sdl2;

use std::collections::HashMap;

use sdl2::keyboard::Keycode;

pub struct Keyboard {
    pub keys: [bool; 16], //HashMap<Keycode, bool>
    key_index_map: HashMap<Keycode, usize>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let key_indexes = [
            (Keycode::Num1, 1),
            (Keycode::Num2, 2),
            (Keycode::Num3, 3),
            (Keycode::C, 0xC),

            (Keycode::Q, 4),
            (Keycode::W, 5),
            (Keycode::E, 6),
            (Keycode::R, 0xD),

            (Keycode::A, 7),
            (Keycode::S, 8),
            (Keycode::D, 9),
            (Keycode::F, 0xE),

            (Keycode::Z, 0xA),
            (Keycode::X, 0x0),
            (Keycode::C, 0xB),
            (Keycode::V, 0xF),
        ];

        Keyboard { keys: [false; 16], key_index_map: HashMap::from(key_indexes) }
    }

    pub fn register_key_event(&mut self, keycode: Keycode, is_key_press: bool) {
        let key_index = match self.key_index_map.get(&keycode) {
            Some(value) => *value,
            None => return
        };

        self.keys[key_index] = is_key_press;
    }
}