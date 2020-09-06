use std::fmt;

pub struct Memory {
    mem: [u8; 4096],
}


impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}

impl fmt::Debug for Memory {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for i in 0..self.mem.len() {
            write!(f, "{:#x}", self.mem[i]);
        }
        std::result::Result::Ok(())
    }
}