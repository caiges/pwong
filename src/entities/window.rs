extern crate sdl2;

use crate::find_sdl_gl_driver;

pub struct Window {
  width: i32,
  height: i32,
  window: sdl2::video::Window,
}

impl Window {
  pub fn new(width: i32, height: i32) -> Window {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
      .window("Window", width as u32, height as u32)
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
