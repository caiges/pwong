extern crate sdl2;
extern crate pwong;

use std::thread;

use sdl2::event::{Event, WindowEventId};
use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer, RenderDrawer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

use pwong::entities::paddle::{Paddle, PaddleDirection};
use pwong::entities::keymap::{KeyPressMap};

fn draw_paddle(drawer: &mut RenderDrawer, paddle: &mut Paddle) {
    drawer.set_draw_color(Color::RGB(255, 157, 0));
    paddle.move_it();
    drawer.draw_rect(Rect::new(paddle.x, paddle.y, paddle.width, paddle.height));
}

fn draw(drawer: &mut RenderDrawer, paddle1: &mut Paddle, paddle2: &mut Paddle) {
    drawer.set_draw_color(Color::RGB(0, 0, 0));
    drawer.clear();
    draw_paddle(drawer, paddle1);
    draw_paddle(drawer, paddle2);
    drawer.present();
}

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new(&sdl_context, "PWONG", WindowPos::PosCentered, WindowPos::PosCentered, 1200, 800, RESIZABLE) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, SOFTWARE) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut keymap = KeyPressMap::new();

    let mut p1 = Paddle::new(0, 40, 800, 40, 100);
    let mut p2 = Paddle::new(760, 40, 800, 40, 100);

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        // Limit to 60 FPS
        thread::sleep_ms(17);

        let mut was_resized = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                Event::Window { win_event_id: WindowEventId::Resized, .. } => { was_resized = true },
                Event::KeyDown{ keycode, .. } => keymap.press(keycode),
                Event::KeyUp{ keycode, .. } => keymap.release(keycode),
                _ => {}
            }
        }

        if was_resized {
            let win_properties = renderer.window_properties(&event_pump).unwrap();
            let (win_width, win_height) = win_properties.get_size();
            if win_width != p2.x + p2.width {
                p2.x = win_width - p2.width;
            }
            if win_height != p1.max_y {
                p1.max_y = win_height;
                p2.max_y = win_height;
            }

            if win_height < p1.y + p1.height {
                p1.y = win_height - p1.height;
            }
            if win_height < p2.y + p2.height {
                p2.y = win_height - p2.height;
            }
        }

        // Do game-y things

        // Clear and redraw
        let mut drawer = renderer.drawer();

        let p1_key = keymap.last_pressed(&[KeyCode::A, KeyCode::Z]);
        p1.direction = match p1_key {
            KeyCode::A => PaddleDirection::UP,
            KeyCode::Z => PaddleDirection::DOWN,
            _ => PaddleDirection::NONE
        };
        let p2_key = keymap.last_pressed(&[KeyCode::Quote, KeyCode::Slash]);
        p2.direction = match p2_key {
            KeyCode::Quote => PaddleDirection::UP,
            KeyCode::Slash => PaddleDirection::DOWN,
            _ => PaddleDirection::NONE
        };

        draw(&mut drawer, &mut p1, &mut p2);
    }
}
