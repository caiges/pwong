extern crate sdl2;

use super::keymap::KeyPressMap;
use super::paddle::Paddle;
use super::court::Court;
use super::ball::Ball;

use self::sdl2::keycode::KeyCode;
use self::sdl2::event::{Event, WindowEventId};

pub struct Game {
    court: Court,
    players: [Paddle; 2],
    score: [i32; 2],
    paused: bool,
    running: bool,
    keymap: KeyPressMap
}

impl Game {
    pub fn new() -> Game {
        let mut court = Court::new(0, 0, 1200, 800);
        let mut p1 = Paddle::new(0, 40, 800, 40, 100);
        let mut p2 = Paddle::new(760, 40, 800, 40, 100);

        Game{players: [p1, p2], score: [0, 0], paused: false, court: court, running: true, keymap: KeyPressMap::new()}
    }

    pub fn run(&mut self) {
        while self.running {
            self.capture_events();
            self.draw();
        }
    }

    pub fn capture_events(&mut self) {
        // let mut event_pump = sdl_context.event_pump();
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => self.quit(),
        //         Event::KeyDown{ keycode: KeyCode::Space, .. } => self.pause(),
        //         Event::KeyDown{ keycode, .. } => self.keymap.press(keycode),
        //         Event::KeyUp{ keycode, .. } => self.keymap.release(keycode),
        //         _ => {}
        //     }
        // }
    }

    pub fn draw(&mut self) {
        // self.court.draw();
        // for player in self.players.iter() {
        //     player.move_it();
        //     player.draw();
        // }
    }

    pub fn reset(&mut self) {
        // self.score = [0,0];
        // for player in self.players.iter() {
        //     player.reset();
        // }

        // self.draw()
    }

    pub fn pause(&mut self) {
        self.paused = ! self.paused;
    }
}