#![allow(dead_code)]

use winit::{event::ElementState, keyboard::Key};

#[derive(Default)]
pub struct GameInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl GameInput {
    pub fn update(&mut self, key: Key, state: ElementState) -> bool {
        let value = match state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        if let Key::Character(name) = key {
            match name.as_str() {
                "w" => {
                    self.up = value;
                    true
                }

                "s" => {
                    self.left = value;
                    true
                }

                "a" => {
                    self.right = value;
                    true
                }

                "d" => {
                    self.down = value;
                    true
                }

                _ => false,
            }
        } else {
            false
        }
    }
}
