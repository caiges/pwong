extern crate sdl2;

use entities::bounds::BoundingBox;

use self::sdl2::rect::Rect;


static DEFAULT_VELOCITY: f32 = 5f32;
static MAX_VELOCITY: f32 = 40f32;
static MULTIPLIER_UP: f32 = -1f32;
static MULTIPLIER_DOWN: f32 = 1f32;
static ACCELERATION_FACTOR: f32 = 1.05;

pub enum PaddleDirection {
    UP,
    DOWN,
    NONE,
}

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub max_y: i32,
    pub width: i32,
    pub height: i32,
    pub velocity: f32,
    pub direction: PaddleDirection,
    pub multiplier: f32,
    pub bounding_box: BoundingBox,
}

impl Paddle {
    pub fn new(x: i32, y: i32, max_y: i32, width: i32, height: i32) -> Paddle {
        Paddle {
            x: x,
            y: y,
            max_y: max_y,
            width: width,
            height: height,
            velocity: DEFAULT_VELOCITY,
            direction: PaddleDirection::NONE,
            multiplier: 0f32,
            bounding_box: BoundingBox::new(x, y, width, height),
        }
    }

    pub fn get_rect(&mut self) -> Rect {
        return Rect::new(self.x, self.y, self.width as u32, self.height as u32);
    }

    pub fn update(&mut self) {
        let multiplier = match self.direction {
            PaddleDirection::UP => MULTIPLIER_UP,
            PaddleDirection::DOWN => MULTIPLIER_DOWN,
            PaddleDirection::NONE => 0f32,
        };

        if (self.multiplier < 0f32) != (multiplier < 0f32) || multiplier == 0f32 {
            self.velocity = DEFAULT_VELOCITY;
        }

        self.multiplier = multiplier;

        let new_y = self.y + (multiplier * self.velocity) as i32;
        if new_y < 0 {
            self.y = 0;
        } else if new_y > self.max_y - self.height {
            self.y = self.max_y - self.height;
        } else {
            self.y = new_y;

            let new_velocity = self.velocity * ACCELERATION_FACTOR;
            if new_velocity <= MAX_VELOCITY {
                self.velocity = new_velocity;
            } else {
                self.velocity = MAX_VELOCITY;
            }
        }

        // Update bounding box location
        self.bounding_box.update_position(self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_and_acceleration() {
        let mut p1 = Paddle::new(0, 40, 800, 40, 100);
        assert!(p1.velocity > 0f32);

        let mut last_y = p1.y;
        let mut last_vel = p1.velocity;

        p1.move_it();
        assert!(p1.velocity == last_vel);
        assert!(p1.y == last_y);

        p1.direction = PaddleDirection::DOWN;
        p1.move_it();
        assert!(p1.velocity > last_vel);
        assert!(p1.y > last_y);

        last_y = p1.y;
        last_vel = p1.velocity;

        p1.move_it();
        assert!(p1.velocity > last_vel);
        assert!(p1.y > last_y);

        last_y = p1.y;
        last_vel = p1.velocity;

        p1.direction = PaddleDirection::NONE;

        p1.move_it();
        assert!(p1.velocity == last_vel);
        assert!(p1.y == last_y);

        p1.direction = PaddleDirection::UP;

        p1.move_it();
        assert!(p1.velocity < last_vel);
        assert!(p1.y < last_y);

        last_y = p1.y;

        p1.move_it();
        assert!(p1.velocity > 5f32);
        assert!(p1.y < last_y);
    }
}
