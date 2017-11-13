extern crate num;
extern crate sdl2;
extern crate time;

use self::sdl2::keyboard::Keycode;

static KEY_COUNT: usize = 235;

pub struct KeyPressMap {
    pub pressed: [u64; 235]
}

impl KeyPressMap {
    pub fn new() -> KeyPressMap {
        KeyPressMap{pressed: [0u64; 235]}
    }

    fn key_to_index(&mut self, key: Keycode) -> usize {
        return key as usize;
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

    pub fn last_pressed(&mut self, keys: &[Keycode]) -> Option<Keycode> {
        let mut last_key: Option<Keycode> = None;
        let mut last_time = 0u64;

        for key in keys {
            let i = self.key_to_index(*key);
            if self.pressed[i] > last_time {
                last_key = Some(*key);
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
        match map.last_pressed(&[Keycode::A, Keycode::B]) {
            Some(key) => assert!(key == Keycode::A),
            None => assert!(false)
        }

        map.press(Keycode::B);
        assert!(map.is_pressed(Keycode::A) == true);
        assert!(map.is_pressed(Keycode::B) == true);
        match map.last_pressed(&[Keycode::A, Keycode::B]) {
            Some(key) => assert!(key == Keycode::B),
            None => assert!(false)
        }

        map.release(Keycode::A);
        assert!(map.is_pressed(Keycode::A) == false);
        assert!(map.is_pressed(Keycode::B) == true);
        match map.last_pressed(&[Keycode::A, Keycode::B]) {
            Some(key) => assert!(key == Keycode::B),
            None => assert!(false)
        }
    }
}