use std::fmt;

///
/// Memory of the Chip8 Virtual Machine struct.
pub struct Memory {
    mem: [u8; 4096],
}


impl Memory {

    /// Creates and returns a new `Memory` struct.
    /// 
    /// # Returns
    ///
    /// A new `Memory` struct.
    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096],
        }
    }

    /// Writes the given byte in the memory at the given address
    /// 
    /// # Parameters
    ///
    /// - `address`: The memory address of the byte to write
    /// - `value`: The byte to write in memory
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    /// Reads the bytes at the given address
    /// 
    /// # Parameters
    ///
    /// - `address`: The memory address of the byte to read
    /// 
    /// # Returns
    /// 
    /// The memory value at address `address`
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