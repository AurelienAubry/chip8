use crate::memory::Memory;

/// Address at which ROMS are loaded in memory
pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    /// CPU registers v0..vf
    vx : [u8; 16],
    /// Memory address register
    i : u16,
    /// Program Counter
    pc : u16,
    /// Stack Pointer
    sp : u8,
    /// Stack
    stack : [u16; 16]
}


impl CPU {

    /// Creates and returns a new `CPU` struct.
    /// 
    /// # Returns
    ///
    /// A new `CPU` struct.
    pub fn new() -> CPU {
        CPU {
            vx : [0; 16],
            i : 0,
            pc : PROGRAM_START,
            sp : 0,
            stack : [0;16], 
        }        
    }

    pub fn cycle(&mut self, memory : &Memory) {
        let opcode : u16 = self.fetch(memory);
        self.decode_and_run(opcode);
    }

    /// Fetch instruction from memory.
    /// 
    /// # Parameters
    /// 
    /// - `memory`: The memory of the Virtual Machine
    /// 
    /// # Returns
    ///
    /// A new `CPU` struct.
    fn fetch(&self, memory : &Memory) -> u16 {
        let mut opcode : u16 = (memory.read_byte(self.pc) as u16) << 8; 
        opcode |= memory.read_byte(self.pc + 1) as u16;

        opcode
    }

    fn decode_and_run(&mut self, opcode : u16) {

        let nnn : u16 = opcode & 0x0FFF;
        let n : u8 = (opcode & 0x000F) as u8;
        let x : u8 = (opcode & 0x0F00) as u8;
        let y : u8 = (opcode & 0x00F0) as u8;
        let kk : u8 = (opcode & 0x00FF) as u8;

        match (opcode & 0xF000) >> 12 {
            0x0 => {
                match kk {
                    0xE0 => self.cls(),
                    0xEE => self.ret(),
                    _ => panic!("Unknown instruction {:#X}", opcode)
                }
            },

            0x1 => self.jp(nnn),
            0x2 => self.call(nnn),
            0x3 => self.se_x_kk(x, kk),
            0x4 => self.sne_x_kk(x, kk),
            0x5 => self.se_x_y(x, y),
            0x6 => self.ld(x, kk),
            0x7 => self.add_x_kk(x, kk),

            _ => panic!("Unknown instruction {:#X}", opcode)
        }
    }

    /// CLS: Clear the display.
    /// TODO
    fn cls(&mut self) {
        print!("Clear screen!");
        self.pc += 2;
    }

    /// RET:  Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, 
    /// then subtracts 1 from the stack pointer.
    fn ret(&mut self) {
        // TODO check if "-1" is needed
        self.pc = self.stack[(self.sp - 1) as usize];
        self.sp -= 1;
        self.pc += 2;
    }

    /// JP: Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn jp(&mut self, nnn : u16) {
        self.pc = nnn;
    }

    /// CALL: Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of 
    /// the stack. The PC is then set to nnn.
    fn call(&mut self, nnn : u16) {
        // TODO check if increment before or after
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    /// SE: Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, 
    /// increments the program counter by 2.
    fn se_x_kk(&mut self, x : u8, kk : u8) {
       if self.read_register(x) == kk {
           self.pc += 2;
       }
       self.pc += 2;
    }

    /// SNE: Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, 
    /// increments the program counter by 2.
    fn sne_x_kk(&mut self, x : u8, kk : u8) {
        if self.read_register(x) != kk {
            self.pc += 2;
        }
        self.pc += 2;
     }

    /// SE: Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, 
    /// increments the program counter by 2.
    fn se_x_y(&mut self, x : u8, y : u8) {
        if self.read_register(x) == self.read_register(y) {
            self.pc += 2;
        }
        self.pc += 2;
     }

    /// LD: Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    /// 
    /// # Parameters
    /// 
    /// - `x` : the register number
    /// - `kk` the value to write in the register
    fn ld(&mut self, x : u8, kk : u8) {
        self.write_register(x, kk);
        self.pc += 2;
    }

    /// ADD: Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx. 
    fn add_x_kk(&mut self, x : u8, kk : u8) {
        let vx_value = self.read_register(x);
        self.write_register(x, vx_value + kk);
        self.pc += 2;
    }


    fn write_register(&mut self, x : u8, value : u8) {
        self.vx[x as usize] = value; 
    }

    fn read_register(&self, x : u8) -> u8 {
        self.vx[x as usize]
    }

}