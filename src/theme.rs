use tui::style::{Color, Style};

use crate::THEME;

#[inline]
pub fn style() -> Style {
    Style::default().bg(THEME.background)
}

pub struct Theme {
    pub background: Color,
    pub unfinished: Color,
    pub finished: Color,
    pub loss: Color,
    pub text_normal: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_dark: Color,
    pub border_primary: Color,
    pub border_secondary: Color,
    pub border_axis: Color,
    pub focused: Color,
    pub unfocused: Color,
}
