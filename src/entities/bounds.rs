pub struct BoundingBox {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32
}

impl BoundingBox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> BoundingBox {
        BoundingBox{x: x, y: y, width: width, height: height}
    }
    pub fn updatePosition(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
