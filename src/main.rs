extern crate sdl2;
extern crate pwong;

use std::thread;

use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer, RenderDrawer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;

use pwong::entities::paddle::{Paddle};

fn draw_paddle(drawer: &mut RenderDrawer, paddle: &mut Paddle) {
    drawer.set_draw_color(Color::RGB(0, 0, 0));
    drawer.clear();
    drawer.set_draw_color(Color::RGB(255, 157, 0));
    drawer.draw_rect(Rect::new(paddle.x, paddle.y, paddle.width, paddle.height));
    drawer.present();
}

fn draw(drawer: &mut RenderDrawer, paddle1: &mut Paddle, paddle2: &mut Paddle) {
    draw_paddle(drawer, paddle1);
    draw_paddle(drawer, paddle2);
}

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new("PWONG", WindowPos::PosCentered, WindowPos::PosCentered, 1200, 800, RESIZABLE) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, SOFTWARE) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut p1 = Paddle::new(0, 40, 800, 40, 100);
    let mut p2 = Paddle::new(760, 40, 800, 40, 100);

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                Event::KeyDown { keycode: KeyCode::A, .. } => {
                    p1.up();
                },
                Event::KeyUp { keycode: KeyCode::A, .. } => {
                    p1.stop();
                },
                Event::KeyDown { keycode: KeyCode::Z, .. } => {
                    p1.down();
                },
                Event::KeyUp { keycode: KeyCode::Z, .. } => {
                    p1.stop();
                },
                Event::KeyDown { keycode: KeyCode::Quote, .. } => {
                    p2.up();
                },
                Event::KeyUp { keycode: KeyCode::Quote, .. } => {
                    p2.stop();
                },
                Event::KeyDown { keycode: KeyCode::Slash, .. } => {
                    p2.down();
                },
                Event::KeyUp { keycode: KeyCode::Slash, .. } => {
                    p2.stop();
                },
                _ => {}
            }
        }
        // Do game-y things

        // Clear and redraw
        thread::sleep_ms(17);

        let mut drawer = renderer.drawer();

        draw(&mut drawer, &mut p1, &mut p2);
        //draw(&mut drawer, &mut p2);
        //move_paddle(&mut drawer, &mut p1);
        //move_paddle(&mut drawer, &mut p2);

        // Draw the queued up renders in the backbuffer
        //drawer.present();
    }
}
