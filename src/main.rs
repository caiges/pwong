extern crate pwong;
extern crate sdl2;

use std::collections::HashMap;
use std::time::Duration;
use std::thread;

use self::sdl2::pixels::Color;
use pwong::entities::main_menu::MainMenu;
use pwong::entities::theme::Theme;
use pwong::entities::window::Window;
use pwong::entities::game::Game;
use pwong::event;
use pwong::Scene;
use pwong::find_sdl_gl_driver;

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

    /*
     * 1. Start pwong.
     * 2. Render main menu.
     * 3. Start game.
     * 4. Render pause menu.
     * 5. Exit pause menu, resume game.
     *
     * # Modal
     *
     * Major modes such as in-game versus being at the main menu will use their own
     * primary loops for event handling.
     *
     * Minor modes, such as a puse menu during gameplay, will live entirely within
     * the primary loop for the major mode. That means that the event pump used by
     * the major mode will handle minor mode interactions.
     *
     * # Drawing
     *
     * To acheive this behavior we will introduce a render stack. Active items
     * on the render stack will have their render method called which will be
     * passed a canvas that they can draw on.
     *
     * # Event Handling
     *
     * Events will be handle
     *
     */
    let window = Window::new(INITIAL_WIDTH, INITIAL_HEIGHT, &video_subsystem);
    /*let mut game = Game::new(
p        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
        &ttf_context,
    );
    game.run(window, &mut event_pump);*/

    let mut canvas = window
            .window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    match canvas.window().gl_set_context_to_current() {
        Err(why) => panic!("{:?}", why),
        Ok(_) => {}
    }

    // Register our custom events.
    event::start_game(&event_subsystem);

    let mut main_menu = MainMenu::new(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
        &theme,
    );

    let mut game = Game::new(
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        &sdl_context,
        &event_subsystem,
        &video_subsystem,
        &theme,
    );

    let mut scenes: HashMap<&str, Box<dyn Scene>> = HashMap::new();
    scenes.insert("main_menu", Box::new(main_menu));
    scenes.insert("game", Box::new(game));

    let activeScene = "main_menu";

loop {
    let scene: &mut std::boxed::Box<dyn pwong::Scene> = scenes.get_mut(activeScene).unwrap();
        for event in event_pump.poll_iter() {
            scene.capture_event(event);
        }

        scene.update();
        scene.wipe(&mut canvas);
        scene.draw(&mut canvas);
        scene.audio();

        thread::sleep(Duration::from_millis(17));
    }
}
