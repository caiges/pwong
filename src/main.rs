extern crate sdl2;
extern crate pwong;

use std::thread;

use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer, RenderDrawer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

use pwong::entities::paddle::{Paddle};
use pwong::entities::keymap::{KeyPressMap};

fn draw_paddle(drawer: &mut RenderDrawer, paddle: &mut Paddle) {
    drawer.set_draw_color(Color::RGB(255, 157, 0));
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

    let mut p1 = Paddle::new(0, 40, 40, 40, 100);
    let mut p2 = Paddle::new(760, 40, 40, 40, 100);
    let movement_multiplier = 80;
    let mut keymap = KeyPressMap::new();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        // Limit to 60 FPS
        thread::sleep_ms(17);

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                Event::KeyDown{ keycode, .. } => keymap.press(keycode),
                Event::KeyUp{ keycode, .. } => keymap.release(keycode),
                _ => {}
            }
        }
        // Do game-y things

        // Clear and redraw
        let mut drawer = renderer.drawer();
        
        draw(&mut drawer, &mut p1, &mut p2);
    }
}
