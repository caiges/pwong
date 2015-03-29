pub struct Ball {
  x: i16,
  y: i16
}

impl Ball {
  fn position(&self) -> [i16; 2] {
    [self.x, self.y]
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