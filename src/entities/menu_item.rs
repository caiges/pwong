use super::textbox::TextBox;
use super::theme::Theme;

pub struct MenuItem<'a> {
  content: TextBox<'a, 'a>,
}

impl<'a> MenuItem<'a> {
  pub fn new(theme: &'a Theme<'a, 'a>, content: &'a str) -> MenuItem<'a> {
    let tb = TextBox::new(&theme, content);

    MenuItem { content: tb }
  }
}
