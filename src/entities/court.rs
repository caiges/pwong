pub struct Court {
    pub width: i32,
    pub height: i32
}

impl Court {
    pub fn new(width: i32, height: i32) -> Court {
        Court{width: width, height: height}
    }
}