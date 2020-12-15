extern crate sdl2;

use self::sdl2::pixels::Color;
use self::sdl2::rwops::RWops;
use self::sdl2::ttf::Font;
use self::sdl2::ttf::Sdl2TtfContext;


pub struct Theme<'ttf, 'a> {
    pub color: Color,
    pub font: Font<'ttf, 'a>,
    pub font_size: u16,
}

impl<'ttf, 'a> Theme<'ttf, 'a> {
    pub fn new(color: Color, font_bytes: &'a[u8], font_size: u16, ttf_context: &'ttf Sdl2TtfContext) -> Theme<'ttf, 'a> {
        let ttf_rwops = RWops::from_bytes(font_bytes).unwrap();
        let font = ttf_context.load_font_from_rwops(ttf_rwops, font_size).unwrap();

        Theme {
            color: color,
            font: font,
            font_size: font_size,
        }
    }
}
