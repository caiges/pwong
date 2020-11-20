extern crate sdl2;

use self::sdl2::pixels::Color;
use self::sdl2::rwops::RWops;
use self::sdl2::ttf::Font;
use self::sdl2::ttf::Sdl2TtfContext;


pub struct Theme<'a> {
    pub color: Color,
    pub font: Font<'a, 'a>,
    pub font_size: u16,
}

impl<'a> Theme<'a> {
    pub fn new(color: Color, font_bytes: &'a[u8], font_size: u16, ttf_context: &'a Sdl2TtfContext) -> Theme<'a> {
        let ttf_rwops = RWops::from_bytes(font_bytes).unwrap();
        let font = ttf_context.load_font_from_rwops(ttf_rwops, font_size).unwrap();

        Theme {
            color: color,
            font: font,
            font_size: font_size,
        }
    }
}
