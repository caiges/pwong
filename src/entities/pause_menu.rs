extern crate gl;
extern crate sdl2;

use super::keymap::KeyPressMap;
use super::menu_item::MenuItem;
use super::textbox::TextBox;
use super::theme::Theme;

use crate::audio;
use crate::event;
use crate::Scene;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::mixer;
use self::sdl2::pixels::Color;
use self::sdl2::render::WindowCanvas;
use self::sdl2::Sdl;

use std::env;

pub struct PauseMenu<'a> {
    running: bool,
    paused: bool,
    keymap: KeyPressMap,
    audio_player: audio::player::Player<'a>,
    sdl_context: &'a Sdl,
    event_subsystem: &'a sdl2::EventSubsystem,
    video_subsystem: &'a sdl2::VideoSubsystem,
    theme: &'a Theme<'a, 'a>,
    items: Vec<MenuItem<'a>>,
    selected_item: usize,
    pwong_label: TextBox<'a, 'a>,
}

impl<'a> PauseMenu<'a> {
    pub fn new(
        width: i32,
        height: i32,
        sdl_context: &'a Sdl,
        event_subsystem: &'a sdl2::EventSubsystem,
        video_subsystem: &'a sdl2::VideoSubsystem,
        theme: &'a Theme<'a, 'a>,
    ) -> PauseMenu<'a> {
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

        let pwong_label = TextBox::new(&theme, "PWong!!!");

        // Menu entities.
        let resume_game = MenuItem::new(
            &theme,
            "Resume Game",
            Some(crate::event::resume_game(&event_subsystem)),
        );
        let main_menu = MenuItem::new(
            &theme,
            "Quit to Main Menu",
            Some(crate::event::main_menu(&event_subsystem)),
        );
        let quit_pwong = MenuItem::new(
            &theme,
            "Quit Pwong",
            Some(crate::event::quit_game(&event_subsystem)),
        );

        let items: Vec<MenuItem<'a>> = vec![resume_game, main_menu, quit_pwong];

        PauseMenu {
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
        if self.selected_item == self.items.len() - 1 {
            self.selected_item = 0;
            return;
        }

        self.selected_item += 1
    }

    pub fn previous_item(&mut self) {
        if self.selected_item == 0 {
            self.selected_item = self.items.len() - 1;
            return;
        }

        self.selected_item -= 1;
    }

    pub fn activate_item(&mut self) {
        match &self.items[self.selected_item].event {
            Some(e) => {
                self.event_subsystem.push_event(e.clone());
            }
            None => {}
        };
    }
}

impl<'a> Scene for PauseMenu<'a> {
    fn handle_resize(&mut self, window_width: i32, window_height: i32) {}

    fn capture_event(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.event_subsystem
                    .push_event(crate::event::resume_game(&self.event_subsystem));
            }
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

    fn update(&mut self) {}

    fn wipe(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        self.pwong_label.render(canvas, 200, 100);

        canvas.set_draw_color(self.theme.color);
        let coords = [100, 200];
        for (i, item) in self.items.iter_mut().enumerate() {
            let x = coords[0];
            let y = coords[1] + item.content.height as i32 * 2 * i as i32;
            item.content.render(canvas, x, y);

            if self.selected_item == i {
                let points = crate::item_marker_points(x - 30, y + item.content.height as i32 / 2);
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

    fn pause(&mut self) {}

    fn resume(&mut self) {}
}
