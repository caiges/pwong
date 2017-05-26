extern crate sdl2;
extern crate pwong;

use pwong::entities::game::Game;

static INITIAL_HEIGHT: u32 = 800;
static INITIAL_WIDTH: u32 = 1200;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let mut game = Game::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    game.run(sdl_context);
}