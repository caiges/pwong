extern crate pwong;
extern crate sdl2;

use pwong::entities::game::Game;
use pwong::entities::window::Window;

static INITIAL_HEIGHT: i32 = 800;
static INITIAL_WIDTH: i32 = 1200;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_subsystem = sdl_context.event().unwrap();

    let window = Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, &video_subsystem);
    let mut game = Game::new(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
    );
    game.run(window);
}
