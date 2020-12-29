extern crate sdl2;

use super::textbox::TextBox;
use super::theme::Theme;

pub struct MenuItem<'a> {
  pub content: TextBox<'a, 'a>,
  pub event: Option<sdl2::event::Event>,
}

impl<'a> MenuItem<'a> {
  pub fn new(theme: &'a Theme<'a, 'a>, content: &'a str, event: Option<sdl2::event::Event>) -> MenuItem<'a> {
    let tb = TextBox::new(&theme, content);

    MenuItem { content: tb, event: event }
  }
}
