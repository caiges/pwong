extern crate pwong;
extern crate sdl2;

use self::sdl2::pixels::Color;
use pwong::entities::main_menu::MainMenu;
use pwong::entities::theme::Theme;
use pwong::entities::window::Window;

static INITIAL_HEIGHT: i32 = 800;
static INITIAL_WIDTH: i32 = 1200;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_subsystem = sdl_context.event().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Setup the theme.
    let color = Color::RGB(255, 157, 0);
    let font_size = 36;
    let font_bytes = include_bytes!("OpenSans-Regular.ttf");
    let theme = Theme::new(color, font_bytes, font_size, &ttf_context);

    let window = Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, &video_subsystem);
    /*let mut game = Game::new(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
        &ttf_context,
    );
    game.run(window, &mut event_pump);*/
    let mut main_menu = MainMenu::new(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
        &theme,
    );
    main_menu.run(window, &mut event_pump);
}
