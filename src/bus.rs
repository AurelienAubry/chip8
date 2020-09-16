use crate::memory::Memory;
use crate::keyboard::Keyboard;
use crate::cpu;
use crate::display::Display;
use crate::display;

pub struct Bus {
    mem : Memory,
    display : Display,
    keyboard : Keyboard,
    /// Delay timer
    dt : u8,
    /// Sound timer
    st : u8
}

impl Bus {

    pub fn new() -> Bus {

        Bus {
            mem : Memory::new(),
            display : Display::new(),
            keyboard : Keyboard::new(),
            dt : 0,
            st : 0,
        }
    }

    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        for i in 0..buffer.len() {
            self.mem.write_byte(cpu::PROGRAM_START + (i as u16), buffer[i]);
        }
    }

    pub fn mem_write_byte(&mut self, address: u16, value: u8) {
        self.mem.write_byte(address, value);
    }

    pub fn mem_read_byte(&self, address: u16) -> u8 {
        self.mem.read_byte(address)
    }


    pub fn draw_byte(&mut self, x : usize, y : usize, value : u8) -> bool {
        self.display.draw_byte(x, y, value)
    }

    pub fn get_display_buffer(&mut self) -> [u32; display::WIDTH * display::HEIGHT] {
        self.display.get_display_buffer()
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    pub fn set_pressed_key(&mut self, key: Option<u8>) {
        self.keyboard.set_pressed_key(key)
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keyboard.get_pressed_key()
    }

    pub fn is_key_pressed(&self, key : u8) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn set_dt(&mut self, value : u8) {
        self.dt = value;
    }

    pub fn set_st(&mut self, value : u8) {
        self.st = value;
    }

    pub fn get_dt(&self) -> u8 {
        self.dt
    }

    pub fn get_st(&self) -> u8 {
        self.st
    }

    pub fn dec_dt(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn dec_st(&mut self) {
        if self.st > 0 {
            self.st -= 1;
        }
    }

    

}
