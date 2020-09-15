use crate::cpu::CPU;
use crate::bus::Bus;
use std::fmt;

use crate::cpu;

/// Chip8 Virtual Machine struct
pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
}

impl Chip8 {

    /// Creates and returns a new `Chip8` struct.
    /// 
    /// # Returns
    ///
    /// A new `Chip8` struct.
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: CPU::new()
        }
    }

    /// Load a ROM in Chip8 memory
    /// 
    /// # Parameters
    ///
    /// - `buffer`: The bytes of the ROM to load in memory
    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        self.bus.load_rom(buffer);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.cycle(&mut self.bus);
            self.bus.update_display();
            for i in 0..90000{

            }
        }
    }

}

/*
impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "MEMORY\n{:?}", self.mem);
        std::result::Result::Ok(())
    }
}*/