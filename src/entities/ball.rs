const SPEED: i32 = 1;

pub struct Ball {
  pub x: i32,
  pub y: i32,
  pub r: i32,
  pub vx: i32,
  pub vy: i32
}

impl Ball {
  pub fn new(x: i32, y: i32, r: i32, vx: i32, vy: i32) -> Ball {
    Ball{x: x, y: y, r: r, vx: vx, vy: vy}
  }

  fn position(&self) -> [i32; 2] {
    [self.x, self.y]
  }

  pub fn update_position(&mut self) {
    self.x += self.vx * SPEED;
    self.y += self.vy * SPEED;
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
