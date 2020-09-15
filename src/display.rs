extern crate minifb;

use minifb::{Window, WindowOptions};


pub const WIDTH : usize = 64;
pub const HEIGHT : usize = 32;


pub struct Display {
    buffer : [u32; WIDTH * HEIGHT],
    display_buffer : [u32; WIDTH * HEIGHT],
    window : Window,
}

impl Display {

    pub fn new() -> Display {

        let mut window = Window::new(
            "Chip8", 
            WIDTH * 10, 
            HEIGHT * 10, 
            WindowOptions::default()).unwrap();

        let display = Display {
            buffer : [0; WIDTH * HEIGHT],
            display_buffer : [0; WIDTH * HEIGHT],
            window : window,
        };
        
        display
    }

    fn get_pixel_index(&self, x : usize, y : usize) -> usize {
        (y % HEIGHT) * WIDTH + (x % WIDTH)
    }

    pub fn set_pixel(&mut self, x : usize, y : usize, value : u8){
        let index = self.get_pixel_index(x, y);
        self.buffer[index] ^= value as u32;
    }

    pub fn draw_byte(&mut self, x : usize, y : usize, value : u8) {
        for i in 0..8 {
            self.set_pixel(x + 8 - i, y, (value >> i) & 0x1);
        }
        
    }

    pub fn update(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = self.get_pixel_index(x, y);
                let pixel = self.buffer[index];
                let pixel_color = match pixel {
                    0x0 => 0x0,
                    0x1 => 0xFFFFFF,
                    _ => { 
                        println!("{:#x}", pixel);
                        0xFF33CC
                    }
                };
                self.display_buffer[index] = pixel_color;
            }
        }

        self.window.update_with_buffer(&self.display_buffer, WIDTH, HEIGHT);
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
        self.display_buffer = [0; WIDTH * HEIGHT];
    }


}