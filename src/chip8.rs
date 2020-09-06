
use crate::memory::Memory;
use std::fmt;

pub struct Chip8 {
    mem: Memory,
}

pub const PROGRAM_START: u16 = 0x200;

impl Chip8 {

    pub fn new() -> Chip8 {
        Chip8 {
            mem: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        for i in 0..buffer.len() {
            self.mem.write_byte(PROGRAM_START + (i as u16), buffer[i]);
        }
    }

}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "MEMORY\n{:?}", self.mem);
        std::result::Result::Ok(())
    }
}