extern crate sdl2;

use crate::find_sdl_gl_driver;

pub struct Window {
  pub width: i32,
  pub height: i32,
  pub window: sdl2::video::Window,
}

impl Window {
  pub fn new(width: i32, height: i32, video_subsystem: &sdl2::VideoSubsystem) -> Window {
    let window = video_subsystem
      .window("PWong", width as u32, height as u32)
      .opengl()
      .position_centered()
      .resizable()
      .build()
      .unwrap();

    Window {
      width: width,
      height: height,
      window: window,
    }
  }
}
