static DEFAULT_VELOCITY : f32 = 10f32;
static MULTIPLIER_UP : f32 = -1f32;
static MULTIPLIER_DOWN : f32 = 1f32;
static ACCELERATION_FACTOR : f32 = 1.2;

pub enum PaddleDirection {
    UP,
    DOWN,
    NONE
}

pub struct Paddle {
	pub x: i32,
	pub y: i32,
	pub max_y: i32,
	pub width: i32,
	pub height: i32,
    pub direction: PaddleDirection,
    pub velocity: f32
}

impl Paddle {
	pub fn new(x: i32, y: i32, max_y: i32, width: i32, height: i32) -> Paddle {
		Paddle{x: x, y: y, max_y: max_y, width: width, height: height, direction: PaddleDirection::NONE, velocity: DEFAULT_VELOCITY}
	}

	fn position(&self) -> [i32; 2] {
		[self.x, self.y]
	}

    pub fn set_direction(&mut self, direction: PaddleDirection) {
        match direction {
            PaddleDirection::NONE => {
                self.velocity = DEFAULT_VELOCITY;
                self.direction = direction;
            },
            _ => self.direction = direction
        };
    }

    pub fn move_it(&mut self) {
        let multiplier = match self.direction {
            PaddleDirection::UP => MULTIPLIER_UP,
            PaddleDirection::DOWN => MULTIPLIER_DOWN,
            PaddleDirection::NONE => return,
        };
        let new_y = self.y + (multiplier * self.velocity) as i32;
        println!("Old Y = {}", self.y);
        println!("Velocity = {}", self.velocity);
        println!("Multiplier = {}", multiplier);
        println!("New y = {}", new_y);
        if new_y < 0 {
            self.y = 0;
            self.velocity = DEFAULT_VELOCITY
        }
        else if new_y > self.max_y - self.height {
            self.y = self.max_y - self.height;
            self.velocity = DEFAULT_VELOCITY
        }
        else {
            self.y = new_y;
            // accelerate this biotch!
            self.velocity *= ACCELERATION_FACTOR;
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