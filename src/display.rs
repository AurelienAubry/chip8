extern crate minifb;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

/// Chip8 Display struct
pub struct Display {
    /// The buffer that contains pixels information (on or off)
    buffer: [u32; WIDTH * HEIGHT],
    /// The buffer that contains pixels colors (0xFFFFFF or 0x000000)
    display_buffer: [u32; WIDTH * HEIGHT],
}

impl Display {
    /// Creates and returns a new `Display` struct.
    ///
    /// # Returns
    ///
    /// A new `Display` struct.
    pub fn new() -> Display {
        let display = Display {
            buffer: [0; WIDTH * HEIGHT],
            display_buffer: [0; WIDTH * HEIGHT],
        };
        display
    }

    /// Gets pixel index in the pixel buffer given its (x, y) coordinates
    ///
    /// # Parameters
    ///
    /// - `x`: The x coordinate of the pixel
    /// - `y`: The y coordinate of the pixel
    ///
    /// # Returns
    ///
    /// The index of the pixel in the pixel buffer
    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    /// Sets the pixel (x, y) value
    ///
    /// # Parameters
    ///
    /// - `x`: The x coordinate of the pixel
    /// - `y`: The y coordinate of the pixel
    /// - `value`: The value of the pixel (0x0 or 0x1)
    ///
    /// # Returns
    ///
    /// `true` if the operation erased the pixel
    fn set_pixel(&mut self, x: usize, y: usize, value: u8) -> bool {
        let index = self.get_pixel_index(x, y);
        let prev_value = self.buffer[index];
        let pixel_value = prev_value ^ value as u32;

        let mut erased = false;
        // The pixel was set and has been erased
        if prev_value == 0x1 && pixel_value == 0x0 {
            erased = true;
        }

        // Update pixel in the pixel buffer
        self.buffer[index] = pixel_value;

        // Set pixel color in the display_buffer
        self.set_pixel_color(index, pixel_value);

        erased
    }

    /// Sets the pixel color based on its index in `display_buffer`
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the pixel in `display_buffer`
    /// - `pixel_value`: The value of the pixel (0x0 or 0x1)
    fn set_pixel_color(&mut self, index: usize, pixel_value: u32) {
        let pixel_color = match pixel_value {
            0x0 => 0x0,
            0x1 => 0xFFFFFF,
            _ => {
                println!("{:#x}", pixel_value);
                0xFF33CC
            }
        };
        self.display_buffer[index] = pixel_color;
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
        let mut erased = false;
        let mut coord_x;
        let coord_y = y % HEIGHT;

        let mut offset = 7;
        // Set each pixel of the byte
        for i in 0..8 {
            coord_x = (x + i) % WIDTH;
            let pixel_value = (value >> offset) & 0x1;

            // Check if any pixel has been erased
            if self.set_pixel(coord_x, coord_y, pixel_value) {
                erased = true;
            }
            offset -= 1;
        }
        erased
    }
    /// Gets the display buffer
    ///
    /// # Returns
    ///
    /// The display buffer
    pub fn get_display_buffer(&self) -> [u32; WIDTH * HEIGHT] {
        self.display_buffer
    }

    /// Clears the display
    pub fn clear(&mut self) {
        for i in 0..(WIDTH * HEIGHT) {
            self.buffer[i] = 0;
            self.display_buffer[i] = 0;
        }
    }
}
