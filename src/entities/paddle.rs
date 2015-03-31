pub struct Paddle {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32
}

impl Paddle {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Paddle {
		Paddle{x: x, y: y, width: width, height: height}
	}

	fn position(&self) -> [i32; 2] {
		[self.x, self.y]
	}

	pub fn up(&mut self, mult: i32) {
		if(self.y > 0) {
			self.y -= 1 * mult;
		}
	}

	pub fn down(&mut self, mult: i32) {
		self.y += 1 * mult;
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