extern crate gl;
extern crate sdl2;

use super::ball::Ball;
use super::court::Court;
use super::keymap::KeyPressMap;
use super::paddle::{Paddle, PaddleDirection};
use super::window::Window;
use crate::audio;
use crate::find_sdl_gl_driver;

use self::sdl2::event::{Event, WindowEvent};
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use self::sdl2::render::TextureQuery;
use self::sdl2::render::WindowCanvas;
use self::sdl2::rwops::RWops;
use self::sdl2::EventPump;
use self::sdl2::Sdl;

use std::env;
use std::thread;
use std::time::Duration;

static PADDLE_WIDTH: i32 = 40;
static PADDLE_HEIGHT: i32 = 100;
static BALL_RADIUS: i32 = 15;
static INITIAL_BALL_VX: i32 = -4;
static INITIAL_BALL_VY: i32 = 0;

pub struct Game<'a> {
    running: bool,
    paused: bool,
    score: [i32; 2],
    court: Court,
    players: [Paddle; 2],
    ball: Ball,
    keymap: KeyPressMap,
    audio_player: audio::player::Player<'a>,
    sdl_context: self::sdl2::Sdl,
    video_subsystem: self::sdl2::VideoSubsystem,
}

impl<'a> Game<'a> {
    pub fn new(
        width: i32,
        height: i32,
        sdl_context: Sdl,
        event_subsystem: sdl2::EventSubsystem,
        video_subsystem: sdl2::VideoSubsystem,
    ) -> Game<'a> {
        // Open mixer.
        mixer::open_audio(
            44_100,
            mixer::DEFAULT_FORMAT,
            mixer::DEFAULT_CHANNELS,
            1_024,
        )
        .unwrap();

        // Our own systems.
        let pack = env::var("PWONG_ASSET_PACK").unwrap_or("caige".to_string());
        let audio_player = audio::player::Player::new(pack);

        // Game entities.
        let court = Court::new(width, height);
        let paddle_y = height / 2 - PADDLE_HEIGHT / 2;
        let p1 = Paddle::new(0, paddle_y, height, PADDLE_WIDTH, PADDLE_HEIGHT);
        let p2 = Paddle::new(
            width - PADDLE_WIDTH,
            paddle_y,
            height,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        let ball_x = width / 2 - BALL_RADIUS / 2;
        let ball_y = height / 2 - BALL_RADIUS / 2;
        let ball = Ball::new(
            ball_x,
            ball_y,
            BALL_RADIUS,
            INITIAL_BALL_VX,
            INITIAL_BALL_VY,
            event_subsystem.clone(),
        );

        Game {
            running: true,
            paused: true,
            score: [0, 0],
            court: court,
            players: [p1, p2],
            ball: ball,
            keymap: KeyPressMap::new(),
            audio_player: audio_player,
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
        }
    }

    pub fn run(&mut self, window: Window) {
        let mut canvas = window
            .window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        gl::load_with(|name| self.video_subsystem.gl_get_proc_address(name) as *const _);
        match canvas.window().gl_set_context_to_current() {
            Err(why) => panic!("{:?}", why),
            Ok(_) => {}
        }

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        while self.running {
            self.capture_events(&mut event_pump);
            self.update();
            self.check_for_score();
            self.wipe(&mut canvas);
            self.draw(&mut canvas);
            self.audio();

            thread::sleep(Duration::from_millis(17));
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit(),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.pause(),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => self.reset(),
                Event::KeyDown { keycode, .. } => self.keymap.press(keycode.unwrap()),
                Event::KeyUp { keycode, .. } => self.keymap.release(keycode.unwrap()),
                Event::Window {
                    win_event: WindowEvent::Resized(data1, data2),
                    ..
                } => self.handle_resize(data1, data2),
                Event::User { code: 456, .. } => {
                    self.audio_player.add("ball_collision".to_string())
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        if !self.paused {
            match self.keymap.last_pressed(&[Keycode::A, Keycode::Z]) {
                Some(key) => {
                    match key {
                        Keycode::A => self.players[0].direction = PaddleDirection::UP,
                        Keycode::Z => self.players[0].direction = PaddleDirection::DOWN,
                        _ => {}
                    };
                }
                None => self.players[0].direction = PaddleDirection::NONE,
            };

            match self.keymap.last_pressed(&[Keycode::Quote, Keycode::Slash]) {
                Some(key) => {
                    match key {
                        Keycode::Quote => self.players[1].direction = PaddleDirection::UP,
                        Keycode::Slash => self.players[1].direction = PaddleDirection::DOWN,
                        _ => {}
                    };
                }
                None => self.players[1].direction = PaddleDirection::NONE,
            };

            for player in self.players.iter_mut() {
                player.update()
            }

            self.ball
                .update(&self.players[0], &self.players[1], self.court.height);
        }
    }

    pub fn wipe(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        let color = Color::RGB(255, 157, 0);
        let font_size = 36;
        let margin = 20i32;

        canvas.set_draw_color(color);
        for player in self.players.iter_mut() {
            match canvas.draw_rect(player.get_rect()) {
                Err(why) => panic!("{:?}", why),
                Ok(_) => {}
            }
        }
        let points = self.ball.get_points();
        match canvas.draw_points(&points[..]) {
            Err(why) => panic!("{:?}", why),
            Ok(_) => {}
        }

        let texture_creator = canvas.texture_creator();

        let ttf_bytes = include_bytes!("../OpenSans-Regular.ttf");
        let ttf_rwops = RWops::from_bytes(ttf_bytes).unwrap();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let sdl_font = ttf_context
            .load_font_from_rwops(ttf_rwops, font_size)
            .unwrap();

        for (i, score) in self.score.iter().enumerate() {
            let surface = sdl_font
                .render(&score.to_string())
                .blended(color)
                .map_err(|e| e.to_string())
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            let TextureQuery { width, height, .. } = texture.query();

            let x = if i == 0 {
                margin
            } else {
                self.court.width - width as i32 - margin
            };
            let score_box = Rect::new(x, margin, width, height);
            canvas.copy(&texture, None, Some(score_box)).unwrap();
        }

        canvas.present();
    }

    pub fn audio(&mut self) {
        self.audio_player.play().unwrap();
        if !self.paused {
            self.audio_player
                .play_music("orchestra".to_string(), self.paused)
                .unwrap();
        } else {
            self.audio_player.pause_music();
        }
    }

    // In lieu of a more structured player type and event system, monitor the x coordinate of the ball, score for the appropriate player
    pub fn check_for_score(&mut self) {
        if self.ball.x - self.ball.r <= 0 {
            self.score(1);
            self.audio_player.add("score".to_string());
        } else if self.ball.x + self.ball.r >= self.court.width {
            self.score(0);
            self.audio_player.add("score".to_string());
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
        self.audio_player.rewind_music();
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
