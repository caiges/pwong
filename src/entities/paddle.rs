pub struct Paddle {
	x: i8,
	y: i8,
	width: f64,
	height: f64
}

pub fn hello() -> String {
	"Hello!".to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hello() {
		assert_eq!("Hello!".to_string(), hello());
	}

	#[test]
	fn test_basic_paddle() {
		let p1 = Paddle{x: 15, y: 15, width: 20.0, height: 20.0};
		assert!(p1.x > 0);
	}
}