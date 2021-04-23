use super::block;
use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Paragraph, StatefulWidget, Widget, Wrap};

pub struct ItemState {
    pub text: String,
    pub done: bool,
}

impl ItemState {
    pub fn new(text: String) -> ItemState {
        ItemState { text, done: false }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn done(&self) -> bool {
        self.done
    }
}

pub struct ItemWidget {}

impl StatefulWidget for ItemWidget {
    type State = ItemState;

    fn render(self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        block::new(&format!("{}", state.done)).render(area, buf);
        area = add_padding(area, 1, PaddingDirection::All);
        area = add_padding(area, 1, PaddingDirection::Left);
        area = add_padding(area, 1, PaddingDirection::Right);

        let mut chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(100)].as_ref())
            .split(area);

        Paragraph::new(state.text())
            .style(style().fg(THEME.text_normal()))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(chunk[0], buf);
    }
}
