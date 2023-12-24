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
            (Keycode::Num1, 0),
            (Keycode::Num2, 1),
            (Keycode::Num3, 2),
            (Keycode::Num4, 3),
            (Keycode::Q, 4),
            (Keycode::W, 5),
            (Keycode::E, 6),
            (Keycode::R, 7),
            (Keycode::A, 8),
            (Keycode::S, 9),
            (Keycode::D, 0xA),
            (Keycode::F, 0xB),
            (Keycode::Z, 0xC),
            (Keycode::X, 0xD),
            (Keycode::C, 0xE),
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