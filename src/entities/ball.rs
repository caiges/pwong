extern crate sdl2;

use std::f32::consts;

use entities::bounds::BoundingBox;
use entities::paddle::Paddle;

use self::sdl2::rect::Point;

const SPEED: i32 = 4;
const MAXBOUNCEANGLE: f32 = (5.0 * consts::PI) / 12.0;

pub struct Ball {
	pub x: i32,
	pub y: i32,
	pub r: i32,
	pub vx: i32,
	pub vy: i32,
	pub bounding_box: BoundingBox,
}

impl Ball {
	pub fn new(x: i32, y: i32, r: i32, vx: i32, vy: i32) -> Ball {
		Ball {
			x: x,
			y: y,
			r: r,
			vx: vx,
			vy: vy,
			bounding_box: BoundingBox::new(x - r, y - r, r * 2, r * 2),
		}
	}

	pub fn get_points(&mut self) -> Vec<Point> {
		let mut points = Vec::new();
		let mut f = 1 - self.r;
		let mut ddf_x = 1;
		let mut ddf_y = -2 * self.r;
		let mut x = 0;
		let mut y = self.r;
		points.push(Point::new(self.x, self.y + self.r));
		points.push(Point::new(self.x, self.y - self.r));
		points.push(Point::new(self.x + self.r, self.y));
		points.push(Point::new(self.x - self.r, self.y));

		while x < y {
			if f >= 0 {
				y -= 1;
				ddf_y += 2;
				f += ddf_y;
			}
			x += 1;
			ddf_x += 2;
			f += ddf_x;
			points.push(Point::new(self.x + x, self.y + y));
			points.push(Point::new(self.x - x, self.y + y));
			points.push(Point::new(self.x + x, self.y - y));
			points.push(Point::new(self.x - x, self.y - y));
			points.push(Point::new(self.x + y, self.y + x));
			points.push(Point::new(self.x - y, self.y + x));
			points.push(Point::new(self.x + y, self.y - x));
			points.push(Point::new(self.x - y, self.y - x));
		}
		return points;
	}

	// Determine the y value of intersection and return it
	pub fn intersection(&self, paddle: &Paddle) -> i32 {
		let intersect: i32;
		if self.y < paddle.y {
			intersect = paddle.y;
		} else if self.y > paddle.y + paddle.height {
			intersect = paddle.y + paddle.height;
		} else {
			intersect = self.y;
		}
		intersect
	}

	// Calculate the bounce angle used for reflection
	pub fn bounce_angle(&self, paddle: &Paddle) -> f32 {
		let intersection_y = self.intersection(&paddle);
		let relative_intersect = (paddle.y + (paddle.height / 2)) - intersection_y;
		let normalized_intersect =
			(relative_intersect as f32 / (paddle.height as f32 / 2 as f32)) as f32;

		return normalized_intersect * MAXBOUNCEANGLE;
	}

	// Update the ball's position and handle any collisions
	pub fn update(&mut self, paddle1: &Paddle, paddle2: &Paddle, max_y: i32) {
		self.x += self.vx * SPEED;
		self.y += self.vy * SPEED;

		// Prevent the ball from traveling past the court.
		if self.y - self.r < 0 {
			self.y = self.r;
		}

		if self.y + self.r > max_y {
			self.y = max_y - self.r;
		}

		self
			.bounding_box
			.update_position(self.x - self.r, self.y - self.r);

		// If there is a collision, set new vectors
		if self.bounding_box.collides_with(&paddle1.bounding_box) {
			let bounce_angle = self.bounce_angle(&paddle1);

			self.vx = (bounce_angle.cos() * SPEED as f32) as i32;
			self.vy = -(bounce_angle.sin() * SPEED as f32) as i32;
		} else if self.bounding_box.collides_with(&paddle2.bounding_box) {
			let bounce_angle = self.bounce_angle(&paddle2);

			self.vx = -(bounce_angle.cos() * SPEED as f32) as i32;
			self.vy = -(bounce_angle.sin() * SPEED as f32) as i32;
		}
		if self.bounding_box.y == 0 || self.bounding_box.y + self.bounding_box.height == max_y {
			// Handle the court boundaries
			self.vy = -self.vy;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::paddle::Paddle;
	use super::*;

	#[test]
	fn test_intersection() {
		let paddle = Paddle::new(0, 40, 1000, 10, 100);
		let mut ball = Ball::new(10, 40, 10, 1, 1);

		assert!(ball.intersection(&paddle) == paddle.y);

		ball.y = 50;
		assert!(ball.intersection(&paddle) == ball.y);

		ball.y = 145;
		assert!(ball.intersection(&paddle) == paddle.y + paddle.height);
	}

	#[test]
	fn test_bounce_angle() {
		let paddle = Paddle::new(0, 40, 1000, 10, 100);
		let ball = Ball::new(10, 50, 10, 1, 1);
		let bounce_angle = ball.bounce_angle(&paddle);

		// Remember boys and girls, directly comparing floats is not accurate.
		// For our uses, this level of precision is good enough.
		assert!(bounce_angle > 1.0 && bounce_angle < 1.1);
	}

	#[test]
	fn test_update() {
		let paddle1 = Paddle::new(0, 40, 1000, 10, 100);
		let paddle2 = Paddle::new(0, 1000, 1000, 10, 100);
		let mut ball = Ball::new(12, 60, 15, -1, 0);

		ball.update(&paddle1, &paddle2, 1000);

		// Up and to the right
		assert!(ball.vx == 2 && ball.vy == -2);

		ball.x = 12;
		ball.y = 95;
		ball.vx = -1;
		ball.vy = 0;
		ball.update(&paddle1, &paddle2, 1000);

		// Straight across
		assert!(ball.vx == 3 && ball.vy == 0);

		ball.x = 12;
		ball.y = 110;
		ball.vx = -1;
		ball.vy = 0;
		ball.update(&paddle1, &paddle2, 1000);

		// Down and to the right
		assert!(ball.vx == 3 && ball.vy == 2);
	}
}

#[test]
fn test_update_outside_of_court() {
	let paddle1 = Paddle::new(0, 40, 1000, 10, 100);
	let paddle2 = Paddle::new(0, 1000, 1000, 10, 100);
	let mut ball = Ball::new(12, 0, 2, -1, -10);

	ball.update(&paddle1, &paddle2, 100);
	assert!(ball.y == 2, "{}", ball.y);

	ball.y = 100;
	ball.update(&paddle1, &paddle2, 100);
	assert!(ball.y == 98, "{}", ball.y);
}
