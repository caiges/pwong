extern crate sdl2;
extern crate pwong;

use pwong::entities::game::Game;

static INITIAL_HEIGHT: i32 = 800;
static INITIAL_WIDTH: i32 = 1200;

pub fn main() {
    let mut game = Game::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    game.run();
}