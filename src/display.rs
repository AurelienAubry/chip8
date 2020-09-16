extern crate minifb;


pub const WIDTH : usize = 64;
pub const HEIGHT : usize = 32;


pub struct Display {
    buffer : [u32; WIDTH * HEIGHT],
    display_buffer : [u32; WIDTH * HEIGHT],
}

impl Display {

    pub fn new() -> Display {

        let display = Display {
            buffer : [0; WIDTH * HEIGHT],
            display_buffer : [0; WIDTH * HEIGHT],
        };
        
        display
    }

    fn get_pixel_index(&self, x : usize, y : usize) -> usize {
        (y * WIDTH) + x
    }

    pub fn set_pixel(&mut self, x : usize, y : usize, value : u8) -> bool {
        let index = self.get_pixel_index(x, y);
        let mut erased = false;
        let prev_value = self.buffer[index];
        let pixel_value = prev_value ^ value as u32;

        if prev_value == 0x1 && pixel_value == 0x0 {
            erased = true;
        }

        self.buffer[index] = pixel_value;
        self.set_pixel_color(index, pixel_value);

        erased
    }

    fn set_pixel_color(&mut self, index: usize, pixel_value : u32) {
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

    pub fn draw_byte(&mut self, x : usize, y : usize, value : u8) -> bool{
        let mut erased = false;
        let mut coord_x;
        let coord_y = y % HEIGHT;
        /*for i in 0..8 {
            if self.set_pixel(coord_x + 8 - i , coord_y, (value >> i) & 0x1) {
                erased = true;
            }
        }
        erased*/
        let mut offset = 7;
        for i in 0..8 {
            coord_x = (x + i) % WIDTH;
            let pixel_value = (value >> offset) & 0x1;
            if self.set_pixel(coord_x, coord_y, pixel_value) {
                erased = true;
            } 
            offset -= 1;
        }
        erased
        
    }

    pub fn get_display_buffer(&self) -> [u32; WIDTH * HEIGHT] {
        self.display_buffer
    }

    pub fn clear(&mut self) {
        for i in 0..(WIDTH * HEIGHT) {
            self.buffer[i] = 0;
            self.display_buffer[i] = 0;
        }
    }


}