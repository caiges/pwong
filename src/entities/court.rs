pub struct Court {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Court {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Court {
        Court{x: x, y: y, width: width, height: height}
    }
}