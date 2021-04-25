use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;
use chrono::{DateTime, Local};
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

pub struct ItemState {
    pub slot: usize,
    pub text: String,
    pub date: DateTime<Local>,
    pub done: bool,
}

impl ItemState {
    pub fn new(slot: usize, text: String, date: DateTime<Local>) -> ItemState {
        ItemState {
            slot,
            text,
            date,
            done: false,
        }
    }
}

pub struct ItemWidget {}

impl StatefulWidget for ItemWidget {
    type State = ItemState;

    fn render(self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mark = if state.done { "✓" } else { "x" };
        Block::default()
            .title(Span::styled(
                format!(
                    " Slot: {} | Status: [{}] | Created: {} ",
                    state.slot, mark, state.date
                ),
                if state.done {
                    style().fg(THEME.finished())
                } else {
                    style().fg(THEME.text_normal())
                },
            ))
            .borders(Borders::ALL)
            .border_style(if state.done {
                style().fg(THEME.finished())
            } else {
                style().fg(THEME.border_secondary())
            })
            .render(area, buf);
        area = add_padding(area, 1, PaddingDirection::Top);
        area = add_padding(area, 1, PaddingDirection::Left);
        area = add_padding(area, 1, PaddingDirection::Right);

        let text = vec![Span::styled(
            format!(" Objective: {}", state.text),
            if state.done {
                style().fg(THEME.finished())
            } else {
                style().fg(THEME.text_normal())
            },
        )];

        Paragraph::new(Spans::from(text))
            .style(style())
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
