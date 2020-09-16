use std::cell::RefCell;
use std::rc::Rc;

pub struct Keyboard {
    pressed_key: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { 
            pressed_key: None 
        }
    }

    pub fn set_pressed_key(&mut self, key : Option<u8>) {
        self.pressed_key = key;
        //println!("PRESS {:?}", self.pressed_key);
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.pressed_key
    }

    pub fn is_key_pressed(&self, key_code : u8) -> bool{
        if let Some(key) = self.pressed_key {
            key == key_code
        } else {
            false
        }
    }
}