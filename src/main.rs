extern crate pwong;
extern crate sdl2;

use pwong::entities::game::Game;

static INITIAL_HEIGHT: i32 = 800;
static INITIAL_WIDTH: i32 = 1200;

pub fn main() {
    let mut game = Game::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    game.run();
}
