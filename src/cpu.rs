use crate::bus::Bus;
use rand;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

/// Address at which ROMS are loaded in memory
pub const PROGRAM_START: u16 = 0x200;

/// Chip8 CPU struct
pub struct CPU {
    /// CPU registers v0..vf
    vx: [u8; 16],
    /// Memory address register
    i: u16,
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u8,
    /// Stack
    stack: [u16; 16],
    /// Random number generator
    rng: rand::rngs::ThreadRng,
}

impl CPU {
    /// Creates and returns a new `CPU` struct.
    ///
    /// # Returns
    ///
    /// A new `CPU` struct.
    pub fn new() -> CPU {
        CPU {
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
            rng: thread_rng(),
        }
    }

    /// Performs one CPU cycle : fetches / decodes / runs an instruction
    ///
    /// # Parameter
    ///
    /// - `bus`: The Chip8 bus
    pub fn cycle(&mut self, bus: &mut Bus) {
        let opcode: u16 = self.fetch(bus);
        self.decode_and_run(opcode, bus);
    }

    /// Fetches instruction from memory.
    ///
    /// # Parameters
    ///
    /// - `bus`: The Chip8 bus
    ///
    /// # Returns
    ///
    /// The instruction at PC
    fn fetch(&mut self, bus: &mut Bus) -> u16 {
        let mut opcode: u16 = (bus.mem_read_byte(self.pc) as u16) << 8;
        opcode |= bus.mem_read_byte(self.pc + 1) as u16;
        self.pc += 2;
        opcode
    }

    /// Decodes and runs the input instruction
    /// Check http://devernay.free.fr/hacks/chip8/C8TECH10.HTM for more info
    ///
    /// # Parameters
    ///
    /// - `opcode`: The instruction to run
    /// - `bus`: The Chip8 bus
    ///
    /// # Returns
    ///
    /// A new `CPU` struct.
    fn decode_and_run(&mut self, opcode: u16, bus: &mut Bus) {
        let nnn: u16 = opcode & 0x0FFF;
        let n: u8 = (opcode & 0x000F) as u8;
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let kk: u8 = (opcode & 0x00FF) as u8;

        // Decode and execute the instruction
        match (opcode & 0xF000) >> 12 {
            0x0 => match kk {
                0xE0 => self.cls(bus),
                0xEE => self.ret(),
                _ => panic!("Unknown instruction {:#X}", opcode),
            },

            0x1 => self.jp(nnn),
            0x2 => self.call(nnn),

            0x3 => self.se_x_kk(x, kk),
            0x4 => self.sne_x_kk(x, kk),
            0x5 => self.se_x_y(x, y),
            0x6 => self.ld_x_kk(x, kk),
            0x7 => self.add_x_kk(x, kk),

            0x8 => match n {
                0x0 => self.ld_x_y(x, y),
                0x1 => self.or_x_y(x, y),
                0x2 => self.and_x_y(x, y),
                0x3 => self.xor_x_y(x, y),
                0x4 => self.add_x_y(x, y),
                0x5 => self.sub_x_y(x, y),
                0x6 => self.shr_x(x),
                0x7 => self.subn_x_y(x, y),
                0xE => self.shl_x(x),
                _ => panic!("Unknown instruction {:#X}", opcode),
            },

            0x9 => self.sne(x, y),
            0xA => self.ld_i_nnn(nnn),
            0xB => self.jp_0_nnn(nnn),
            0xC => self.rnd_x_kk(x, kk),
            0xD => self.drw(x, y, n, bus),

            0xE => match kk {
                0x9E => self.skp_x(x, bus),
                0xA1 => self.sknp_x(x, bus),
                _ => panic!("Unknown instruction {:#X}", opcode),
            },

            0xF => match kk {
                0x07 => self.ld_x_dt(x, bus),
                0x0A => self.ld_x_press(x, bus),
                0x15 => self.ld_dt_x(x, bus),
                0x18 => self.ld_st_x(x, bus),
                0x1E => self.add_i_x(x),
                0x29 => self.ld_f_x(x),
                0x33 => self.ld_b_x(x, bus),
                0x55 => self.ld_i_x(x, bus),
                0x65 => self.ld_x_i(x, bus),
                _ => panic!("Unknown instruction {:#X}", opcode),
            },

            _ => panic!("Unknown instruction {:#X}", opcode),
        }
    }

    /// CLS - 00E0: Clear the display.
    fn cls(&mut self, bus: &mut Bus) {
        bus.clear_display();
    }

    /// RET - 00EE:  Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn ret(&mut self) {
        // TODO check if "-1" is needed
        self.pc = self.stack[(self.sp - 1) as usize];
        self.sp -= 1;
    }

    /// JP - 1nnn: Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn jp(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /// CALL - 2nnn: Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of
    /// the stack. The PC is then set to nnn.
    fn call(&mut self, nnn: u16) {
        // TODO check if increment before or after
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    /// SE - 3xkk: Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    fn se_x_kk(&mut self, x: u8, kk: u8) {
        if self.read_register(x) == kk {
            self.pc += 2;
        }
    }

    /// SNE - 4xkk: Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    fn sne_x_kk(&mut self, x: u8, kk: u8) {
        if self.read_register(x) != kk {
            self.pc += 2;
        }
    }

    /// SE - 5xy0: Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal,
    /// increments the program counter by 2.
    fn se_x_y(&mut self, x: u8, y: u8) {
        if self.read_register(x) == self.read_register(y) {
            self.pc += 2;
        }
    }

    /// LD - 6xkk: Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn ld_x_kk(&mut self, x: u8, kk: u8) {
        self.write_register(x, kk);
    }

    /// ADD - 7xkk: Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn add_x_kk(&mut self, x: u8, kk: u8) {
        let vx_value = self.read_register(x);
        self.write_register(x, vx_value.wrapping_add(kk));
    }

    /// LD - 8xy0: Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn ld_x_y(&mut self, x: u8, y: u8) {
        self.write_register(x, self.read_register(y));
    }

    /// OR - 8xy1: Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corresponding bits from two values,
    /// and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn or_x_y(&mut self, x: u8, y: u8) {
        self.write_register(x, self.read_register(x) | self.read_register(y));
    }

    /// AND - 8xy2: Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corresponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn and_x_y(&mut self, x: u8, y: u8) {
        self.write_register(x, self.read_register(x) & self.read_register(y));
    }

    /// XOR - 8xy3: Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corresponding bits from two values, and if the bits are not
    /// both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn xor_x_y(&mut self, x: u8, y: u8) {
        self.write_register(x, self.read_register(x) ^ self.read_register(y));
    }

    /// ADD - 8xy4: Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn add_x_y(&mut self, x: u8, y: u8) {
        let vx = self.read_register(x) as u16;
        let vy = self.read_register(y) as u16;
        let sum = vx + vy;
        self.write_register(x, sum as u8);
        if sum > 0xFF {
            self.write_register(0xF, 1);
        } else {
            self.write_register(0xF, 0);
        }
    }

    /// SUB - 8xy5: Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx,
    /// and the results stored in Vx.
    fn sub_x_y(&mut self, x: u8, y: u8) {
        let vx = self.read_register(x) as i8;
        let vy = self.read_register(y) as i8;
        let sub = vx - vy;
        self.write_register(x, sub as u8);
        if sub > 0 {
            self.write_register(0xF, 1);
        } else {
            self.write_register(0xF, 0);
        }
    }

    /// SHR - 8xy6: Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    fn shr_x(&mut self, x: u8) {
        let vx = self.read_register(x);
        self.write_register(0xF, vx & 0x1);
        self.write_register(x, vx >> 1);
    }

    /// SUBN - 8xy7: Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and
    /// the results stored in Vx.
    fn subn_x_y(&mut self, x: u8, y: u8) {
        let vx = self.read_register(x) as i8;
        let vy = self.read_register(y) as i8;
        let sub = vy - vx;
        self.write_register(x, sub as u8);
        if sub > 0 {
            self.write_register(0xF, 1);
        } else {
            self.write_register(0xF, 0);
        }
    }

    /// SHL - 8xyE: Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn shl_x(&mut self, x: u8) {
        let vx = self.read_register(x);
        self.write_register(0xF, (vx & 0x80) >> 7);
        self.write_register(x, vx << 1);
    }

    /// SNE - 9xy0: Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal,
    /// the program counter is increased by 2.
    fn sne(&mut self, x: u8, y: u8) {
        if self.read_register(x) != self.read_register(y) {
            self.pc += 2;
        }
    }

    /// LD - Annn: Set I = nnn.
    /// The value of register I is set to nnn.
    fn ld_i_nnn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// JP - Bnnn: Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    fn jp_0_nnn(&mut self, nnn: u16) {
        self.pc = (self.read_register(0) as u16) + nnn;
    }

    /// RND - Cxkk : Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with
    /// the value kk. The results are stored in Vx. See instruction 8xy2 for more information on
    /// AND.
    fn rnd_x_kk(&mut self, x: u8, kk: u8) {
        let range = Uniform::from(0..=255);
        let rnd_byte = range.sample(&mut self.rng);
        self.write_register(x, rnd_byte & kk);
    }

    /// DRW - Dxyn : Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it
    /// is outside the coordinates of the display, it wraps around to the opposite side of the
    /// screen.
    fn drw(&mut self, x: u8, y: u8, n: u8, bus: &mut Bus) {
        let mut value;
        let vx = self.read_register(x);
        let vy = self.read_register(y);
        let mut erased = false;
        for index in 0..n {
            value = bus.mem_read_byte(self.i + index as u16);
            if bus.draw_byte(vx as usize, (vy + index) as usize, value) {
                erased = true;
            }
        }

        if erased {
            self.write_register(0xF, 0x1);
        } else {
            self.write_register(0xF, 0x0);
        }
    }

    /// SKP - Ex9E : Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the
    /// down position, PC is increased by 2.
    fn skp_x(&mut self, x: u8, bus: &Bus) {
        let vx = self.read_register(x);
        if bus.is_key_pressed(vx) {
            self.pc += 2;
        }
    }

    /// SKNP - ExA1 : Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the
    /// up position, PC is increased by 2.
    fn sknp_x(&mut self, x: u8, bus: &Bus) {
        let vx = self.read_register(x);
        if !bus.is_key_pressed(vx) {
            self.pc += 2;
        }
    }

    /// LD - Fx07 : Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn ld_x_dt(&mut self, x: u8, bus: &Bus) {
        self.write_register(x, bus.get_dt());
    }

    /// LD - Fx0A : Wait for a key press, store the value of the key in Vx.
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn ld_x_press(&mut self, x: u8, bus: &Bus) {
        if let Some(key) = bus.get_pressed_key() {
            self.write_register(x, key);
        }
    }

    /// LD - Fx15 : Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn ld_dt_x(&mut self, x: u8, bus: &mut Bus) {
        let vx = self.read_register(x);
        bus.set_dt(vx);
    }

    /// LD - Fx18 : Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    fn ld_st_x(&mut self, x: u8, bus: &mut Bus) {
        let vx = self.read_register(x);
        bus.set_st(vx);
    }

    /// ADD - Fx1E : Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_i_x(&mut self, x: u8) {
        self.i += self.read_register(x) as u16;
    }

    /// LD - Fx29 : Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the
    /// value of Vx.
    fn ld_f_x(&mut self, x: u8) {
        let vx = self.read_register(x) as u16;
        self.i = vx * 5;
    }

    /// LD - Fx33 : Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn ld_b_x(&mut self, x: u8, bus: &mut Bus) {
        let vx = self.read_register(x);
        bus.mem_write_byte(self.i, vx / 100);
        bus.mem_write_byte(self.i + 1, (vx % 100) / 10);
        bus.mem_write_byte(self.i + 2, vx % 10);
    }

    /// LD - Fx55 : Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the
    /// address in I.
    fn ld_i_x(&mut self, x: u8, bus: &mut Bus) {
        for v_index in 0..=x {
            let vx = self.read_register(v_index);
            bus.mem_write_byte(self.i + v_index as u16, vx);
        }
    }

    /// LD - Fx65 : Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0
    /// through Vx.
    fn ld_x_i(&mut self, x: u8, bus: &Bus) {
        for v_index in 0..=x {
            let vx = bus.mem_read_byte(self.i + v_index as u16);
            self.write_register(v_index, vx);
        }
    }

    /// Writes the given value in the requested register
    ///
    /// # Parameters
    ///
    /// - `x`: The register id
    /// - `value`: The value to write in the register
    fn write_register(&mut self, x: u8, value: u8) {
        self.vx[x as usize] = value;
    }

    /// Reads and returns the value of the specified register
    ///
    /// # Parameters
    ///
    /// - `x`: The register id
    ///
    /// # Returns
    ///
    /// The value stored in the register x
    fn read_register(&self, x: u8) -> u8 {
        self.vx[x as usize]
    }
}
