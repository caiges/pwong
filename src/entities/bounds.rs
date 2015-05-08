trait BoundingBox {
    fn collides_with(&self, candidate: &BoundingBox) -> bool { 
        ! (self.top() > candidate.bottom() ||
            self.right() < candidate.left() ||
            self.bottom() < candidate.top() ||
            self.left() > candidate.right())
    }
    fn position(&self) -> [i32; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }
    fn top(&self) -> i32 {
        self.y
    }
    fn right(&self) -> i32 {
        self.x + self.width
    }
    fn bottom(&self) -> i32 {
        self.y + self.height
    }
    fn left(&self) -> i32 {
        self.x
    }
}