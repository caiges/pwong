pub struct Paddle {
	x: i16,
	y: i16,
	width: f64,
	height: f64
}

impl Paddle {
	fn position(&self) -> [i16; 2] {
		[self.x, self.y]
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