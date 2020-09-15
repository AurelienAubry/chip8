mod chip8;
mod memory;
mod cpu;
mod bus;
mod display;

use chip8::Chip8;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() ->  io::Result<()> {

    // Create Chip8
    let mut chip8 : Chip8 = Chip8::new();

    // Open and read ROM file
    let mut file = File::open("roms/VERS")?;
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)?;

    // Load ROM in Chip8 memory
    chip8.load_rom(&buffer);
    //println!("{:?}", chip8);

    chip8.run();

    Ok(())

}

