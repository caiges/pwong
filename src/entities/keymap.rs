extern crate num;
extern crate sdl2;
extern crate time;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use self::sdl2::keyboard::Keycode;
use self::num::ToPrimitive;

static KEY_COUNT: usize = 235;

pub struct KeyPressMap {
    pub pressed: [u64; 235]
}

impl KeyPressMap {
    pub fn new() -> KeyPressMap {
        KeyPressMap{pressed: [0u64; 235]}
    }

    fn key_to_index(&mut self, key: Keycode) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return hasher.finish() as usize;
    }

    pub fn press(&mut self, key: Keycode) {
        let i = self.key_to_index(key);
        if i <= KEY_COUNT {
            self.pressed[i] = time::precise_time_ns();
        }
    }

    pub fn release(&mut self, key: Keycode) {
        let i = self.key_to_index(key);
        if i <= KEY_COUNT {
            self.pressed[i] = 0u64;
        }
    }

    pub fn last_pressed(&mut self, keys: &[Keycode]) -> Keycode {
        let mut last_key = Keycode::Unknown;
        let mut last_time = 0u64;
        for key in keys {
            let i = self.key_to_index(*key);
            if self.pressed[i] > last_time {
                last_key = *key;
                last_time = self.pressed[i];
            }
        }
        return last_key;
    }

    pub fn is_pressed(&mut self, key: Keycode) -> bool {
        let i = self.key_to_index(key);
        if self.pressed[i] > 0u64 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::sdl2::keyboard::Keycode;

    #[test]
    fn test_key_mapping() {
        let mut map = KeyPressMap::new();
        map.press(Keycode::A);
        assert!(map.is_pressed(Keycode::A) == true);
        assert!(map.is_pressed(Keycode::B) == false);
        assert!(map.last_pressed(&[Keycode::A, Keycode::B]) == Keycode::A);

        map.press(Keycode::B);
        assert!(map.is_pressed(Keycode::A) == true);
        assert!(map.is_pressed(Keycode::B) == true);
        assert!(map.last_pressed(&[Keycode::A, KeyCode::B]) == Keycode::B);

        map.release(Keycode::A);
        assert!(map.is_pressed(Keycode::A) == false);
        assert!(map.is_pressed(Keycode::B) == true);
        assert!(map.last_pressed(&[Keycode::A, Keycode::B]) == Keycode::B);
    }
}