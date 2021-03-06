use crate::bus::Bus;
use crate::cpu::CPU;
use crate::display;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

/// Delay between two CPU cycles
const CPU_CYLE_TIME: u64 = 1000 / 500;
/// Delay between two display refresh (60fps)
const DISPLAY_TIME: u64 = 1000 / 60;
/// Delay between two pressed key checks
const KEY_TIME: u64 = 1000 / 5;

/// Chip8 Virtual Machine struct
pub struct Chip8 {
    /// The Chip8 CPU
    cpu: CPU,
    /// The Chip8 communication bus
    bus: Bus,
}

impl Chip8 {
    /// Creates and returns a new `Chip8` struct.
    ///
    /// # Returns
    ///
    /// A new `Chip8` struct.
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: CPU::new(),
        }
    }

    /// Load a ROM in Chip8 memory
    ///
    /// # Parameters
    ///
    /// - `buffer`: The bytes of the ROM to load in memory
    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        self.bus.load_rom(buffer);
    }

    /// Run the Chip8
    pub fn run(&mut self) {
        // Create display window
        let mut window = Window::new(
            "Chip8",
            display::WIDTH * 10,
            display::HEIGHT * 10,
            WindowOptions::default(),
        )
        .unwrap();

        // Initialize timers
        let mut last_key_time = Instant::now();
        let mut last_cpu_cyle_time = Instant::now();
        let mut last_display_time = Instant::now();

        // Chip8 loop
        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Get pressed key, if any
            let keys_pressed = window.get_keys();
            let key = match keys_pressed {
                Some(keys) => {
                    if !keys.is_empty() {
                        Some(keys[0])
                    } else {
                        None
                    }
                }
                None => None,
            };

            // Update pressed key
            if Instant::now() - last_key_time >= Duration::from_millis(KEY_TIME) {
                self.bus.set_pressed_key(get_key_code(key));
                last_key_time = Instant::now();
            }

            // Run CPU cycle
            if Instant::now() - last_cpu_cyle_time >= Duration::from_millis(CPU_CYLE_TIME) {
                self.cpu.cycle(&mut self.bus);
                last_cpu_cyle_time = Instant::now();
            }

            // Refresh display
            if Instant::now() - last_display_time >= Duration::from_millis(DISPLAY_TIME) {
                let display_buffer = self.bus.get_display_buffer();
                window
                    .update_with_buffer(&display_buffer, display::WIDTH, display::HEIGHT)
                    .unwrap();
                last_display_time = Instant::now();
                // Update delay and sound timers
                self.bus.dec_dt();
                self.bus.dec_st();
            }
        }
    }
}

/// Gets Chip8 hexa key code associated to the computer keyboard pressed key
///
/// # Parameter
///
/// - `key`: The code of the key pressed on the computer keyboard
///
/// # Returns
///
/// The code of the associated Chip8 keyboard key.
fn get_key_code(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::A) => Some(0x1),
        Some(Key::Z) => Some(0x2),
        Some(Key::E) => Some(0x3),
        Some(Key::R) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::S) => Some(0x5),
        Some(Key::D) => Some(0x6),
        Some(Key::F) => Some(0xD),

        Some(Key::U) => Some(0x7),
        Some(Key::I) => Some(0x8),
        Some(Key::O) => Some(0x9),
        Some(Key::P) => Some(0xE),

        Some(Key::J) => Some(0xA),
        Some(Key::K) => Some(0x0),
        Some(Key::L) => Some(0xB),
        Some(Key::M) => Some(0xF),

        _ => None,
    }
}
