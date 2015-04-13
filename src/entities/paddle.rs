static MAXIMUM_SPEED: f32 = 60f32;
static DIRECTION_UP: i32 = -1;
static DIRECTION_DOWN: i32 = 1;

pub struct Paddle {
	pub x: i32,
	pub y: i32,
	pub max_y: i32,
	pub width: i32,
	pub height: i32,
    pub trajectory: f32
}

impl Paddle {
	pub fn new(x: i32, y: i32, max_y: i32, width: i32, height: i32) -> Paddle {
		Paddle{x: x, y: y, max_y: max_y, width: width, height: height, trajectory: 10f32}
	}

	fn position(&self) -> [i32; 2] {
		[self.x, self.y]
	}

	pub fn up(&mut self) {
        self.move_with_acceleration(DIRECTION_UP)
    }

    pub fn down(&mut self) {
        self.move_with_acceleration(DIRECTION_DOWN)
    }

    pub fn stop(&mut self) {
        if self.trajectory < 0f32 {
            self.trajectory = -10f32
        }
        else {
            self.trajectory = 10f32
        }
    }

    pub fn move_with_acceleration(&mut self, direction: i32) {
        if (self.trajectory < 0f32) != (direction < 0) {
            self.trajectory = (direction * 10) as f32;
        }
        else if self.trajectory.abs() < MAXIMUM_SPEED {
            self.trajectory *= 1.2
        }

        let mut new_y = self.y + self.trajectory as i32;
        if new_y > self.max_y - self.height {
            new_y = self.max_y - self.height;
        }
        if new_y > 0 {
			self.y = new_y;
        }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_paddle() {
		let p1 = Paddle{x: 15, y: 15, width: 20.0, height: 20.0};
		assert!(p1.x > 0);
	}

	#[test]
	fn test_position() {
		let p1 = Paddle{x: 15, y: 15, width: 20.0, height: 20.0};
		assert_eq!([15, 15], p1.position());
	}
}