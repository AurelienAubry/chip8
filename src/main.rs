
mod memory;

use memory::Memory;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() ->  io::Result<()> {
    println!("Hello, world!");
    let mut mem : Memory = Memory::new();

    let mut file = File::open("roms/PONG")?;
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)?;

    load_rom(&mut mem, &buffer);
    println!("{:?}", mem);

    Ok(())

}


fn load_rom(mem: &mut Memory, buffer: &Vec<u8>) {
    let program_start: u16 = 0x200;
    for i in 0..buffer.len() {
        mem.write_byte(program_start + (i as u16), buffer[i]);
    }

}

