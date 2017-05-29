extern crate sdl2;
extern crate gl;

use super::keymap::KeyPressMap;
use super::paddle::{Paddle, PaddleDirection};
use super::court::Court;
use super::ball::Ball;

use self::sdl2::EventPump;
use self::sdl2::keyboard::Keycode;
use self::sdl2::event::{Event, WindowEvent};
use self::sdl2::video::{Window, WindowPos};
use self::sdl2::render::Canvas;
use self::sdl2::pixels::Color;

use std::thread;

static PADDLE_WIDTH: i32 = 40;
static PADDLE_HEIGHT: i32 = 100;
static BALL_RADIUS: i32 = 15;
static INITIAL_BALL_VX: i32 = -4;
static INITIAL_BALL_VY: i32 = 0;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub struct Game {
    running: bool,
    paused: bool,
    score: [i32; 2],
    court: Court,
    players: [Paddle; 2],
    ball: Ball,
    keymap: KeyPressMap,
}


impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let court = Court::new(width, height);
        let paddle_y = height / 2 - PADDLE_HEIGHT / 2;
        let p1 = Paddle::new(0, paddle_y, height, PADDLE_WIDTH, PADDLE_HEIGHT);
        let p2 = Paddle::new(width - PADDLE_WIDTH,
                             paddle_y,
                             height,
                             PADDLE_WIDTH,
                             PADDLE_HEIGHT);
        let ball_x = width / 2 - BALL_RADIUS / 2;
        let ball_y = height / 2 - BALL_RADIUS / 2;
        let ball = Ball::new(ball_x,
                             ball_y,
                             BALL_RADIUS,
                             INITIAL_BALL_VX,
                             INITIAL_BALL_VY);

        Game {
            running: true,
            paused: true,
            score: [0, 0],
            court: court,
            players: [p1, p2],
            ball: ball,
            keymap: KeyPressMap::new(),
        }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current();

        let mut event_pump = sdl_context.event_pump().unwrap();

        while self.running {
            self.capture_events(&mut event_pump);
            self.move_objects();
            self.wipe(&mut canvas);
            self.draw(&mut canvas);
            self.check_for_score();
            thread::sleep_ms(17);
        }
    }

    pub fn handle_resize(&mut self, window_width: i32, window_height: i32) {
        if window_width != self.players[1].x + self.players[1].width {
            self.players[1].x = window_width - self.players[1].width;
        }
        if window_height != self.players[0].max_y {
            self.players[0].max_y = window_height;
            self.players[1].max_y = window_height;
        }

        if window_height < self.players[0].y + self.players[0].height {
            self.players[0].y = window_height - self.players[0].height;
        }
        if window_height < self.players[1].y + self.players[1].height {
            self.players[1].y = window_height - self.players[1].height;
        }

        self.court.width = window_width;
        self.court.height = window_height;
    }

    pub fn capture_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.quit(),
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => self.pause(),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => self.reset(),
                Event::KeyDown { keycode, .. } => self.keymap.press(keycode.unwrap()),
                Event::KeyUp { keycode, .. } => self.keymap.release(keycode.unwrap()),
                Event::Window { win_event: WindowEvent::Resized(data1, data2), .. } => {
                    self.handle_resize(data1, data2)
                }
                _ => {}
            }
        }
    }

    pub fn move_objects(&mut self) {
        if !self.paused {
            let p1_key = self.keymap.last_pressed(&[Keycode::A, Keycode::Z]);

            if p1_key.is_some() {
                self.players[0].direction = match p1_key.unwrap() {
                    Keycode::A => PaddleDirection::UP,
                    Keycode::Z => PaddleDirection::DOWN,
                    _ => PaddleDirection::NONE,
                };
            }

            let p2_key = self.keymap.last_pressed(&[Keycode::Quote, Keycode::Slash]);

            if p2_key.is_some() {
                self.players[1].direction = match p2_key.unwrap() {
                    Keycode::Quote => PaddleDirection::UP,
                    Keycode::Slash => PaddleDirection::DOWN,
                    _ => PaddleDirection::NONE,
                };
            }

            for player in self.players.iter_mut() {
                player.update()
            }

            self.ball
                .update(&self.players[0], &self.players[1], self.court.height);
        }
    }

    pub fn wipe(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 157, 0));
        for player in self.players.iter_mut() {
            canvas.draw_rect(player.get_rect());
        }
        let points = self.ball.get_points();
        canvas.draw_points(&points[..]);
        canvas.present();
    }

    // In lieu of a more structured player type and event system, monitor the x coordinate of the ball, score for the appropriate player
    pub fn check_for_score(&mut self) {
        if self.ball.x - self.ball.r <= 0 {
            self.score(1);
        } else if self.ball.x + self.ball.r >= self.court.width {
            self.score(0);
        }
    }

    // Score for the given player index and reset
    pub fn score(&mut self, player_index: i32) {
        self.score[player_index as usize] += 1;
        self.restart();
    }

    fn reset_entities(&mut self) {
        for player in self.players.iter_mut() {
            player.y = self.court.height / 2 - player.height / 2;
        }
        self.ball.x = self.court.width / 2 - self.ball.r / 2;
        self.ball.y = self.court.height / 2 - self.ball.r / 2;
        self.ball.vx = INITIAL_BALL_VX;
        self.ball.vy = INITIAL_BALL_VY;
    }

    pub fn restart(&mut self) {
        self.reset_entities();
        self.paused = true;
    }

    pub fn reset(&mut self) {
        self.score = [0, 0];
        self.reset_entities();
        self.paused = true;
    }

    pub fn pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}