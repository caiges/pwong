extern crate sdl2;

use super::theme::Theme;

use self::sdl2::rect::Rect;
use self::sdl2::render::Canvas;
use self::sdl2::render::TextureQuery;
use self::sdl2::video::Window;

pub struct TextBox<'ttf, 'a> {
    pub theme: &'a Theme<'ttf, 'a>,
    pub content: &'a str,
    pub width: u32,
    pub height: u32,
}

impl<'ttf, 'a> TextBox<'ttf, 'a> {
    pub fn new(theme: &'a Theme<'ttf, 'a>, content: &'a str) -> TextBox<'ttf, 'a> {
        let (width, height) = theme.font.size_of(content).unwrap();

        TextBox {
            theme: &theme,
            content: content,
            width: width,
            height: height,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let texture_creator = canvas.texture_creator();

        let surface = self
            .theme
            .font
            .render(&self.content)
            .blended(self.theme.color)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let text_box = Rect::new(x, y, width, height);

        canvas.copy(&texture, None, Some(text_box)).unwrap();
    }
}
