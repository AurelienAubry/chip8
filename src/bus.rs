use crate::cpu;
use crate::display;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

/// Chip8 communication Bus struct
pub struct Bus {
    /// Chip8 Memory
    mem: Memory,
    /// Chip8 Display
    display: Display,
    /// Chip8 Keyboard
    keyboard: Keyboard,
    /// Delay timer
    dt: u8,
    /// Sound timer
    st: u8,
}

impl Bus {
    /// Creates and returns a new `Bus` struct.
    ///
    /// # Returns
    ///
    /// A new `Bus` struct.
    pub fn new() -> Bus {
        Bus {
            mem: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            dt: 0,
            st: 0,
        }
    }

    /// Load a ROM in Chip8 memory
    ///
    /// # Parameters
    ///
    /// - `buffer`: The bytes of the ROM to load in memory
    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        for i in 0..buffer.len() {
            self.mem
                .write_byte(cpu::PROGRAM_START + (i as u16), buffer[i]);
        }
    }

    /// Writes the given byte in memory at the giver address
    ///
    /// # Parameters
    ///
    /// - `address`: The memory address where to write the byte
    /// - `value` : The byte to write
    pub fn mem_write_byte(&mut self, address: u16, value: u8) {
        self.mem.write_byte(address, value);
    }

    /// Reads and returns the bytes at the given address
    ///
    /// # Parameters
    ///
    /// - `address`: The memory address of the byte to read
    ///
    /// # Returns
    ///
    /// The memory value at address `address`
    pub fn mem_read_byte(&self, address: u16) -> u8 {
        self.mem.read_byte(address)
    }
    /// Updates the frame buffer to display the given byte (which is part of a sprite)
    ///
    /// # Parameters
    ///
    /// - `x`: The x coordinate of the sprite part
    /// - `y`: The y coordinate of the sprite part
    /// - `value`: The value sprite part
    ///
    /// # Returns
    ///
    /// `true` if the operation erased any pixel
    pub fn draw_byte(&mut self, x: usize, y: usize, value: u8) -> bool {
        self.display.draw_byte(x, y, value)
    }

    /// Gets the display buffer
    ///
    /// # Returns
    ///
    /// The display buffer
    pub fn get_display_buffer(&mut self) -> [u32; display::WIDTH * display::HEIGHT] {
        self.display.get_display_buffer()
    }

    /// Clears the display
    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    /// Sets keyboard pressed key
    ///
    /// # Parameters
    ///
    /// - `key`: The value of the pressed key (an u8 or None)
    pub fn set_pressed_key(&mut self, key: Option<u8>) {
        self.keyboard.set_pressed_key(key)
    }

    /// Gets keyboard pressed key
    ///
    /// # Returns
    ///
    /// The value of the pressed key (an u8 or None)
    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keyboard.get_pressed_key()
    }

    /// Is the given key currently pressed?
    ///
    /// # Parameters
    ///
    /// - `key`: The value of the key to check
    ///
    /// # Returns
    ///
    /// `true` if the key is pressed, else `false`
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    /// Sets delay timer value
    ///
    /// # Parameters
    ///
    /// - `value`: The value of the delay timer to set
    pub fn set_dt(&mut self, value: u8) {
        self.dt = value;
    }

    /// Sets sound timer value
    ///
    /// # Parameters
    ///
    /// - `value`: The value of the sound timer to set
    pub fn set_st(&mut self, value: u8) {
        self.st = value;
    }

    /// Gets delay timer value
    ///
    /// # Returns
    ///
    /// The value of the delay timer
    pub fn get_dt(&self) -> u8 {
        self.dt
    }

    /// Gets sound timer value
    ///
    /// # Returns
    ///
    /// The value of the sound timer
    pub fn get_st(&self) -> u8 {
        self.st
    }

    /// Decrements delay timer
    pub fn dec_dt(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    /// Decrements sound timer
    pub fn dec_st(&mut self) {
        if self.st > 0 {
            self.st -= 1;
        }
    }
}
