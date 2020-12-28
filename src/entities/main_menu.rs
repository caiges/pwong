extern crate gl;
extern crate sdl2;

use super::keymap::KeyPressMap;
use super::textbox::TextBox;
use super::theme::Theme;
use super::window::Window;

use crate::audio;
use crate::Scene;

use self::sdl2::event::{Event, WindowEvent};
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer;
use self::sdl2::pixels::Color;
use self::sdl2::render::WindowCanvas;
use self::sdl2::Sdl;

use std::env;
use std::thread;
use std::time::Duration;

pub struct MainMenu<'a> {
    running: bool,
    paused: bool,
    keymap: KeyPressMap,
    audio_player: audio::player::Player<'a>,
    sdl_context: &'a Sdl,
    event_subsystem: &'a sdl2::EventSubsystem,
    video_subsystem: &'a sdl2::VideoSubsystem,
    theme: &'a Theme<'a, 'a>,
    items: Vec<TextBox<'a, 'a>>,
    selected_item: i32,
    pwong_label: TextBox<'a, 'a>,
}

impl<'a> MainMenu<'a> {
    pub fn new(
        width: i32,
        height: i32,
        sdl_context: &'a Sdl,
        event_subsystem: &'a sdl2::EventSubsystem,
        video_subsystem: &'a sdl2::VideoSubsystem,
        theme: &'a Theme<'a, 'a>,
    ) -> MainMenu<'a> {
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

        let color = Color::RGB(255, 157, 0);
        let font_size = 36;
        let font_bytes = include_bytes!("../OpenSans-Regular.ttf");

        // Menu entities.
        let pwong_label = TextBox::new(&theme, "PWong!!!");
        let resume = TextBox::new(&theme, "Resume");
        let restart = TextBox::new(&theme, "Restart");
        let to_main = TextBox::new(&theme, "Exit to Main Menu");
        let quit_pwong = TextBox::new(&theme, "Quit Pwong");

        let items: Vec<TextBox<'a, 'a>> = vec![resume, restart, to_main, quit_pwong];

        MainMenu {
            running: true,
            paused: true,
            keymap: KeyPressMap::new(),
            audio_player: audio_player,
            sdl_context: sdl_context,
            event_subsystem: event_subsystem,
            video_subsystem: video_subsystem,
            theme: theme,
            items: items,
            selected_item: 0,
            pwong_label: pwong_label,
        }
    }

    pub fn next_item(&mut self) {
        if self.selected_item == self.items.len() as i32 - 1 {
            self.selected_item = 0;
            return;
        }

        self.selected_item += 1
    }

    pub fn previous_item(&mut self) {
        if self.selected_item == 0 {
            self.selected_item = self.items.len() as i32 - 1;
            return;
        }

        self.selected_item -= 1;
    }

    pub fn activate_item(&mut self) {
        if self.selected_item == self.items.len() as i32 - 1 {
            self.quit();
        }
    }

    pub fn quit(&mut self) {
        std::process::exit(0);
    }
}

impl <'a> Scene for MainMenu<'a> {
    fn handle_resize(&mut self, window_width: i32, window_height: i32) {}

    fn capture_event(&mut self, event: sdl2::event::Event) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.next_item(),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.previous_item(),
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => self.activate_item(),
                _ => {}
            }
    }

    fn update(&mut self) {
        /*if !self.paused {
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
        }*/
    }

    fn wipe(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        let margin = 20i32;

        self.pwong_label.render(canvas, 200, 100);

        canvas.set_draw_color(self.theme.color);
        let coords = [100, 200];
        for (i, item) in self.items.iter_mut().enumerate() {
            let x = coords[0];
            let y = coords[1] + item.height as i32 * 2 * i as i32;
            item.render(canvas, x, y);

            if self.selected_item == i as i32 {
                let points = crate::item_marker_points(x - 30, y + item.height as i32 / 2);
                match canvas.draw_points(&points[..]) {
                    Err(why) => panic!("{:?}", why),
                    Ok(_) => {}
                }
            }
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

}
