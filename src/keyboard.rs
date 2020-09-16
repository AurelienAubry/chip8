
/// Chip8 keyboard struct
pub struct Keyboard {
    /// The keyboard key currently pressed
    pressed_key: Option<u8>,
}

impl Keyboard {
    /// Creates and returns a new `Keyboard` struct.
    ///
    /// # Returns
    ///
    /// A new `Keyboard` struct.
    pub fn new() -> Keyboard {
        Keyboard { pressed_key: None }
    }

    /// Sets keyboard pressed key
    ///
    /// # Parameters
    ///
    /// - `key`: The value of the pressed key (an u8 or None)
    pub fn set_pressed_key(&mut self, key: Option<u8>) {
        self.pressed_key = key;
    }

    /// Gets keyboard pressed key
    ///
    /// # Returns
    ///
    /// The value of the pressed key (an u8 or None)
    pub fn get_pressed_key(&self) -> Option<u8> {
        self.pressed_key
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
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let Some(key) = self.pressed_key {
            key == key_code
        } else {
            false
        }
    }
}
