pub struct Paddle {
	x: f64,
	y: f64,
	width: f64,
	height: f64
}

impl Paddle {
	fn position(&self) -> [f64; 2] {
		[self.x, self.y]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_paddle() {
		let p1 = Paddle{x: 15.0, y: 15.0, width: 20.0, height: 20.0};
		assert!(p1.x > 0.0);
	}

	#[test]
	fn test_position() {
		let p1 = Paddle{x: 15.0, y: 15.0, width: 20.0, height: 20.0};
		assert_eq!([15.0, 15.0], p1.position());
	}	
}