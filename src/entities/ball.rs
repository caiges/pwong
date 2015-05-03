use entities::bounds::BoundingBox;
use entities::paddle::Paddle;

const SPEED: i32 = 1;

pub struct Ball {
  pub x: i32,
  pub y: i32,
  pub r: i32,
  pub vx: i32,
  pub vy: i32,
  pub boundingBox: BoundingBox
}

impl Ball {
  pub fn new(x: i32, y: i32, r: i32, vx: i32, vy: i32) -> Ball {
    Ball{x: x, y: y, r: r, vx: vx, vy: vy, boundingBox: BoundingBox::new(x, y, r * 2, r * 2)}
  }

  fn position(&self) -> [i32; 2] {
    [self.x, self.y]
  }

  pub fn update(&mut self, paddle1: &Paddle, paddle2: &Paddle) {
    self.x += self.vx * SPEED;
    self.y += self.vy * SPEED;

    self.boundingBox.updatePosition(self.x, self.y);
    // Update vectors depending on collisions
/*    if self.bounds().collides_with(paddle1) {
        self.vy = self.reflect(paddle1);
    } else if self.bounds().collides_with(paddle2) {
        self.vy = self.reflect(paddle2);    
    }*/
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
