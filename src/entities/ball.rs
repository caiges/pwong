use entities::bounds::BoundingBox;
use entities::paddle::Paddle;

const SPEED: i32 = 1;

pub struct Ball {
  pub x: i32,
  pub y: i32,
  pub r: i32,
  pub vx: i32,
  pub vy: i32,
}

impl BoundingBox for Ball {}

impl Ball {
  pub fn new(x: i32, y: i32, r: i32, vx: i32, vy: i32) -> Ball {
    Ball{x: x, y: y, r: r, vx: vx, vy: vy}
  }

  fn position(&self) -> [i32; 2] {
    [self.x, self.y]
  }

  pub fn update(&mut self, paddle1: &Paddle, paddle2: &Paddle) {
    self.x += self.vx * SPEED;
    self.y += self.vy * SPEED;

    if self.collides_with(&paddle1) {
        self.vx = 5;
        self.vy = 2;
    } else if self.collides_with(&paddle2) {
        self.vx = -5;
        self.vy = -2;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_position() {
    let b1 = Ball{x: 2334, y: 213};
    assert_eq!([2334, 213], b1.position());
  }
}
