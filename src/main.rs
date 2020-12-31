extern crate pwong;
extern crate sdl2;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use self::sdl2::event::Event;
use self::sdl2::pixels::Color;

use pwong::entities::game::Game;
use pwong::entities::main_menu::MainMenu;
use pwong::entities::pause_menu::PauseMenu;
use pwong::entities::theme::Theme;
use pwong::entities::window::Window;
use pwong::event;
use pwong::find_sdl_gl_driver;
use pwong::Scene;

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

    let mut pause_menu = PauseMenu::new(
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
    scenes.insert("pause_menu", Box::new(pause_menu));

    let mut active_scene = "main_menu";

    loop {
        let scene: &mut std::boxed::Box<dyn pwong::Scene> = scenes.get_mut(active_scene).unwrap();
        for event in event_pump.poll_iter() {
            // Handle scene changes.
            match event {
                Event::User { code: 500, .. } => std::process::exit(0),
                Event::Quit { .. } => std::process::exit(0),
                // Pause game.
                Event::User { code: 450, .. } => {
                    scene.pause();
                    active_scene = "pause_menu";
                }
                // Resume game.
                Event::User { code: 451, .. } => {
                    active_scene = "game";
                    event_subsystem.push_event(crate::event::resume_scene(&event_subsystem));
                    break;
                }
                // Resume scene.
                Event::User { code: 452, .. } => {
                    scene.resume();
                }
                // Exit to main menu.
                Event::User { code: 453, .. } => {
                    active_scene = "main_menu";
                }
                // New game.
                Event::User { code: 400, .. } => {
                    active_scene = "game";
                    event_subsystem.push_event(crate::event::reset_game(&event_subsystem));
                    break;
                }
                _ => scene.capture_event(event),
            }
        }

        scene.update();
        scene.wipe(&mut canvas);
        scene.draw(&mut canvas);
        scene.audio();

        thread::sleep(Duration::from_millis(17));
    }
}
