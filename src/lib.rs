extern crate sdl2;

pub mod audio;
pub mod entities;

use self::sdl2::rect::Point;

pub fn find_sdl_gl_driver() -> Option<u32> {
  for (index, item) in sdl2::render::drivers().enumerate() {
    if item.name == "opengl" {
      return Some(index as u32);
    }
  }
  None
}

pub fn item_marker_points(item_x: i32, item_y: i32) -> Vec<Point> {
  let r = 10;
  let mut points = Vec::new();
  let mut f = 1 - r;
  let mut ddf_x = 1;
  let mut ddf_y = -2 * r;
  let mut x = 0;
  let mut y = r;
  points.push(Point::new(item_x, item_y + r));
  points.push(Point::new(item_x, item_y - r));
  points.push(Point::new(item_x + r, item_y));
  points.push(Point::new(item_x - r, item_y));

  while x < y {
    if f >= 0 {
      y -= 1;
      ddf_y += 2;
      f += ddf_y;
    }
    x += 1;
    ddf_x += 2;
    f += ddf_x;
    points.push(Point::new(item_x + x, item_y + y));
    points.push(Point::new(item_x - x, item_y + y));
    points.push(Point::new(item_x + x, item_y - y));
    points.push(Point::new(item_x - x, item_y - y));
    points.push(Point::new(item_x + y, item_y + x));
    points.push(Point::new(item_x - y, item_y + x));
    points.push(Point::new(item_x + y, item_y - x));
    points.push(Point::new(item_x - y, item_y - x));
  }
  return points;
}
