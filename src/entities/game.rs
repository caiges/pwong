extern crate gl;
extern crate sdl2;

use super::ball::Ball;
use super::court::Court;
use super::keymap::KeyPressMap;
use super::paddle::{Paddle, PaddleDirection};
use super::textbox::TextBox;
use super::theme::Theme;

use crate::audio;
use crate::event;
use crate::Scene;

use self::sdl2::event::{Event, WindowEvent};
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer;
use self::sdl2::pixels::Color;
use self::sdl2::render::WindowCanvas;
use self::sdl2::Sdl;

use std::env;

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
    sdl_context: &'a Sdl,
    event_subsystem: &'a sdl2::EventSubsystem,
    video_subsystem: &'a sdl2::VideoSubsystem,
    theme: &'a Theme<'a, 'a>,
}

impl<'a> Game<'a> {
    pub fn new(
        width: i32,
        height: i32,
        sdl_context: &'a Sdl,
        event_subsystem: &'a sdl2::EventSubsystem,
        video_subsystem: &'a sdl2::VideoSubsystem,
        theme: &'a super::theme::Theme,
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
            event_subsystem: event_subsystem,
            video_subsystem: video_subsystem,
            theme: theme,
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

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl<'a> Scene for Game<'a> {
    fn handle_resize(&mut self, window_width: i32, window_height: i32) {
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

    fn capture_event(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.event_subsystem
                    .push_event(crate::event::pause_game(&self.event_subsystem));
            }
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
            Event::User { code: 456, .. } => self.audio_player.add("ball_collision".to_string()),
            _ => {}
        }
    }

    fn update(&mut self) {
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

        self.check_for_score();
    }

    fn wipe(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        let margin = 20i32;

        canvas.set_draw_color(self.theme.color);
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

        for (i, score) in self.score.iter().enumerate() {
            let score_str = &score.to_string();
            let mut score_box = TextBox::new(&self.theme, score_str);
            let x = if i == 0 {
                margin
            } else {
                self.court.width - margin - score_box.width as i32
            };
            let y = margin;
            score_box.render(canvas, x, y);
        }

        canvas.present();
    }

    fn audio(&mut self) {
        self.audio_player.play().unwrap();
        if !self.paused {
            self.audio_player
                .play_music("orchestra".to_string(), self.paused)
                .unwrap();
        } else {
            self.audio_player.pause_music();
        }
    }

    fn pause(&mut self) {
        self.paused = !self.paused;
    }

    fn resume(&mut self) {
        self.paused = !self.paused;
    }
}
