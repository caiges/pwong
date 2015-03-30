extern crate sdl2;

use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer};
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new("PWONG", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, RESIZABLE) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, SOFTWARE) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut drawer = renderer.drawer();
    drawer.set_draw_color(Color::RGB(255, 157, 0));
    drawer.clear();
    drawer.present();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                _ => {}
            }
        }
        // Do game-y things
    }
}
