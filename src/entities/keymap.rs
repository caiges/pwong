extern crate num;
extern crate sdl2;
extern crate time;

use self::sdl2::keycode::KeyCode;
use self::num::ToPrimitive;

static KEY_COUNT : usize = 235;

pub struct KeyPressMap {
    pub pressed: [u64; 235]
}

impl KeyPressMap {
    pub fn new() -> KeyPressMap {
        KeyPressMap{pressed: [0u64; 235]}
    }

    fn key_to_index(&mut self, key: KeyCode) -> usize {
        return key.to_isize().unwrap() as usize
    }

    pub fn press(&mut self, key: KeyCode) {
        let i = self.key_to_index(key);
        if i <= KEY_COUNT {
            self.pressed[i] = time::precise_time_ns();
        }
    }

    pub fn release(&mut self, key: KeyCode) {
        let i = self.key_to_index(key);
        if i <= KEY_COUNT {
            self.pressed[i] = 0u64;
        }
    }

    pub fn last_pressed(&mut self, keys: &[KeyCode]) -> KeyCode {
        let mut last_key = KeyCode::Unknown;
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

    pub fn is_pressed(&mut self, key: KeyCode) -> bool {
        let i = self.key_to_index(key);
        if self.pressed[i] > 0u64 {
            return true
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::sdl2::keycode::KeyCode;

    #[test]
    fn test_key_mapping() {
        let mut map = KeyPressMap::new();
        map.press(KeyCode::A);
        assert!(map.is_pressed(KeyCode::A) == true);
        assert!(map.is_pressed(KeyCode::B) == false);
        assert!(map.last_pressed(&[KeyCode::A, KeyCode::B]) == KeyCode::A);

        map.press(KeyCode::B);
        assert!(map.is_pressed(KeyCode::A) == true);
        assert!(map.is_pressed(KeyCode::B) == true);
        assert!(map.last_pressed(&[KeyCode::A, KeyCode::B]) == KeyCode::B);

        map.release(KeyCode::A);
        assert!(map.is_pressed(KeyCode::A) == false);
        assert!(map.is_pressed(KeyCode::B) == true);
        assert!(map.last_pressed(&[KeyCode::A, KeyCode::B]) == KeyCode::B);
    }
}