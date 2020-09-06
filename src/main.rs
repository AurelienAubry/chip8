
mod chip8;
mod memory;

use chip8::Chip8;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() ->  io::Result<()> {


    // Create Chip8
    let mut chip8 : Chip8 = Chip8::new();

    // Load ROM
    let mut file = File::open("roms/PONG")?;
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)?;

    chip8.load_rom(&buffer);
    println!("{:?}", chip8);

    Ok(())

}

