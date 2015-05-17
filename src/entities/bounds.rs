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
    pub fn update_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn collides_with(&self, candidate: &BoundingBox) -> bool { 
        ! (self.top() > candidate.bottom() ||
            self.right() < candidate.left() ||
            self.bottom() < candidate.top() ||
            self.left() > candidate.right())
    }
    pub fn position(&self) -> [i32; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }
    pub fn top(&self) -> i32 {
        self.y
    }
    pub fn right(&self) -> i32 {
        self.x + self.width
    }
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }
    pub fn left(&self) -> i32 {
        self.x
    }
}
