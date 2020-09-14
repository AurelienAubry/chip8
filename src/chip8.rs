
use crate::memory::Memory;
use crate::cpu::CPU;
use std::fmt;

use crate::cpu;

/// Chip8 Virtual Machine struct
pub struct Chip8 {
    mem: Memory,
    cpu: CPU
}

impl Chip8 {

    /// Creates and returns a new `Chip8` struct.
    /// 
    /// # Returns
    ///
    /// A new `Chip8` struct.
    pub fn new() -> Chip8 {
        Chip8 {
            mem: Memory::new(),
            cpu: CPU::new()
        }
    }

    /// Load a ROM in Chip8 memory
    /// 
    /// # Parameters
    ///
    /// - `buffer`: The bytes of the ROM to load in memory
    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        for i in 0..buffer.len() {
            self.mem.write_byte(cpu::PROGRAM_START + (i as u16), buffer[i]);
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.cycle(&self.mem);
        }
    }

}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "MEMORY\n{:?}", self.mem);
        std::result::Result::Ok(())
    }
}