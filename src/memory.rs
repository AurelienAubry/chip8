use std::fmt;

/// Memory of the Chip8 Virtual Machine struct.
pub struct Memory {
    mem: [u8; 4096],
}

/// Hexa sprites to load in memory at address 0x0
const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];

impl Memory {
    /// Creates and returns a new `Memory` struct.
    ///
    /// # Returns
    ///
    /// A new `Memory` struct.
    pub fn new() -> Memory {
        let mut memory = Memory { mem: [0; 4096] };
        let mut i = 0;
        for sprite in &SPRITES {
            for byte in sprite {
                memory.mem[i] = *byte;
                i += 1;
            }
        }

        memory
    }

    /// Writes the given byte in memory at the giver address
    ///
    /// # Parameters
    ///
    /// - `address`: The memory address where to write the byte
    /// - `value` : The byte to write
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.mem.len() {
            write!(f, "{:#x}", self.mem[i]);
        }
        std::result::Result::Ok(())
    }
}
