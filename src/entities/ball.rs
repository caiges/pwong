use std::f32::{consts};

use entities::bounds::BoundingBox;
use entities::paddle::Paddle;

const SPEED: i32 = 2;
const MAXBOUNCEANGLE: f32 = (5.0 * consts::PI) / 12.0;

pub struct Ball {
	pub x: i32,
	pub y: i32,
	pub r: i32,
	pub vx: i32,
	pub vy: i32,
	pub bounding_box: BoundingBox
}

impl Ball {
	pub fn new(x: i32, y: i32, r: i32, vx: i32, vy: i32) -> Ball {
		Ball{x: x, y: y, r: r, vx: vx, vy: vy, bounding_box: BoundingBox::new(x - r, y - r, r * 2, r * 2)}
	}

	// Determine the y value of intersection and return it
	fn intersection(&self, paddle: &Paddle) -> i32 {
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
	fn bounce_angle(&self, paddle: &Paddle) -> f32 {
		let intersection_y = self.intersection(&paddle);
		let relative_intersect = (paddle.y + (paddle.height / 2)) - intersection_y;
		let normalized_intersect = (relative_intersect as f32 / (paddle.height as f32 / 2 as f32)) as f32;

		return (normalized_intersect * MAXBOUNCEANGLE as f32);
	}

	// Update the balls position and handle any collisions
	pub fn update(&mut self, paddle1: &Paddle, paddle2: &Paddle, window_height: i32) {
		self.x += self.vx * SPEED;
		self.y += self.vy * SPEED;
		self.bounding_box.update_position(self.x - self.r, self.y - self.r);

		// If there is a collision, set new vectors
		if self.bounding_box.collides_with(&paddle1.bounding_box) {
			let bounce_angle = self.bounce_angle(&paddle1);

			self.vx = (bounce_angle.cos() * SPEED as f32).round() as i32;
			self.vy = -(bounce_angle.sin() * SPEED as f32).round() as i32;
		} else if self.bounding_box.collides_with(&paddle2.bounding_box) {
			let bounce_angle = self.bounce_angle(&paddle2);

			self.vx = -(bounce_angle.cos() * SPEED as f32).round() as i32;
			self.vy = (bounce_angle.sin() * SPEED as f32).round() as i32;
		} else if self.y - self.r <= 0 || self.y + self.r >= window_height {
			self.vy = -self.vy;
		}
	}
}
