use crate::memory::Memory;
use crate::cpu;
use crate::display::Display;

pub struct Bus {
    mem : Memory,
    display : Display,
}

impl Bus {

    pub fn new() -> Bus {
        Bus {
            mem : Memory::new(),
            display : Display::new(),
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


    pub fn draw_byte(&mut self, x : usize, y : usize, value : u8) {
        self.display.draw_byte(x, y, value);
    }

    pub fn update_display(&mut self) {
        self.display.update();
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

}
