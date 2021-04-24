use super::block;
use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;
use chrono::{DateTime, Local};
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap};

pub struct ItemState {
    pub text: String,
    pub date: DateTime<Local>,
    pub done: bool,
}

impl ItemState {
    pub fn new(text: String, date: DateTime<Local>) -> ItemState {
        ItemState {
            text,
            date,
            done: false,
        }
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
        Block::default()
            .title(Span::styled(
                format!(" {} ", state.date),
                style().fg(THEME.text_normal()),
            ))
            .borders(Borders::ALL)
            .border_style(style().fg(THEME.border_secondary()))
            .render(area, buf);
        area = add_padding(area, 1, PaddingDirection::Top);

        area = add_padding(area, 1, PaddingDirection::Left);
        area = add_padding(area, 1, PaddingDirection::Right);

        let mark = if state.done { "✓" } else { "x" };

        let text = vec![Span::styled(
            format!("[{}] {}", mark, state.text),
            style().fg(THEME.text_normal()),
        )];

        Paragraph::new(Spans::from(text))
            .style(style())
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
