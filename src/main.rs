extern crate sdl2;
extern crate pwong;

use std::thread;

use sdl2::video::{Window, WindowPos, RESIZABLE};
use sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer, RenderDrawer};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::event::WindowEventId;

use pwong::entities::paddle::{Paddle};
use pwong::entities::ball::{Ball};

fn draw_circle(drawer: &mut RenderDrawer, ball: &mut Ball) {
    drawer.set_draw_color(Color::RGB(255, 157, 0));
    let mut f = 1 - ball.r;
    let mut ddf_x = 1;
    let mut ddf_y = -2 * ball.r;
    let mut x = 0;
    let mut y = ball.r;
    drawer.draw_point(Point::new(ball.x, ball.y + ball.r));
    drawer.draw_point(Point::new(ball.y, ball.y - ball.r));
    drawer.draw_point(Point::new(ball.x + ball.r, ball.y));
    drawer.draw_point(Point::new(ball.x - ball.r, ball.y));
 
    while x < y {
        if f >= 0 { 
            y -= 1;
            ddf_y += 2;
            f += ddf_y;
        }
        x += 1;
        ddf_x += 2;
        f += ddf_x;  
        drawer.draw_point(Point::new(ball.x + x, ball.y + y));
        drawer.draw_point(Point::new(ball.x - x, ball.y + y));
        drawer.draw_point(Point::new(ball.x + x, ball.y - y));
        drawer.draw_point(Point::new(ball.x - x, ball.y - y));
        drawer.draw_point(Point::new(ball.x + y, ball.y + x));
        drawer.draw_point(Point::new(ball.x - y, ball.y + x));
        drawer.draw_point(Point::new(ball.x +  y, ball.y - x));
        drawer.draw_point(Point::new(ball.x - y, ball.y - x));
    }
}

fn draw_paddle(drawer: &mut RenderDrawer, paddle: &mut Paddle) {
    drawer.set_draw_color(Color::RGB(255, 157, 0));
    drawer.draw_rect(Rect::new(paddle.x, paddle.y, paddle.width, paddle.height));
}

fn draw(drawer: &mut RenderDrawer, paddle1: &mut Paddle, paddle2: &mut Paddle, ball: &mut Ball) {
    drawer.set_draw_color(Color::RGB(0, 0, 0));
    drawer.clear();
    draw_paddle(drawer, paddle1);
    draw_paddle(drawer, paddle2);
    draw_circle(drawer, ball);
    drawer.present();
}

pub fn main() {
    let mut window_width = 1200;
    let mut window_height = 800;

    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new(&sdl_context, "PWONG", WindowPos::PosCentered, WindowPos::PosCentered, window_width, window_height, RESIZABLE) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, SOFTWARE) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut p1 = Paddle::new(0, 40, 40, 40, 100);
    let mut p2 = Paddle::new(1160, 40, 40, 40, 100);
    let movement_multiplier = 30;

    let mut b = Ball::new(120, 40, 15, -1, 0);

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
                Event::KeyDown { keycode: KeyCode::A, .. } => {
                    p1.up(movement_multiplier);
                },
                Event::KeyDown { keycode: KeyCode::Z, .. } => {
                    p1.down(movement_multiplier);
                },
                Event::KeyDown { keycode: KeyCode::Quote, .. } => {
                    p2.up(movement_multiplier);
                },
                Event::KeyDown { keycode: KeyCode::Slash, .. } => {
                    p2.down(movement_multiplier);
                },
                Event::Window { win_event_id: WindowEventId::Resized, data1: data1, data2: data2, .. } => {
                    window_width = data1;
                    window_height = data2;
                }
                _ => {}
            }
        }
        // Do game-y things

        // Clear and redraw
        let mut drawer = renderer.drawer();
        
        draw(&mut drawer, &mut p1, &mut p2, &mut b);
        
        // Update positions
        p1.update();
        p2.update();
        b.update(&p1, &p2, window_height);
    }
}
