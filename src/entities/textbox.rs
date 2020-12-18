extern crate sdl2;

use super::court::Court;
use super::theme::Theme;

use self::sdl2::rect::Rect;
use self::sdl2::render::Canvas;
use self::sdl2::render::TextureQuery;
use self::sdl2::video::Window;

pub struct TextBox<'ttf, 'a> {
    pub theme: &'a Theme<'ttf, 'a>,
    pub content: &'a str,
    pub court: &'a Court,
}

impl<'ttf, 'a> TextBox<'ttf, 'a> {
    pub fn new(theme: &'a Theme<'ttf, 'a>, content: &'a str, court: &'a Court) -> TextBox<'ttf, 'a> {
        TextBox {
            theme: &theme,
            content: content,
            court: court,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, x_offset: i32, y_offset: i32) {
        let texture_creator = canvas.texture_creator();

        let surface = self.theme.font.render(&self.content)
            .blended(self.theme.color).map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let x = if x_offset < 0 { self.court.width + x_offset - width as i32 } else { x_offset };
        let y = if y_offset < 0 { self.court.height + height as i32 + y_offset - height as i32 } else { y_offset };
        let text_box = Rect::new(x, y, width, height);

        canvas.copy(&texture, None, Some(text_box)).unwrap();
    }
}
