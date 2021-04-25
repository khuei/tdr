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
    pub expire_date: DateTime<Local>,
    pub done: bool,
    pub selected: bool,
}

impl ItemState {
    pub fn new(
        slot: usize,
        text: String,
        date: DateTime<Local>,
        expire_date: DateTime<Local>,
    ) -> ItemState {
        ItemState {
            slot,
            text,
            date,
            expire_date,
            done: false,
            selected: true,
        }
    }

    fn get_time_offset(&self) -> String {
        let mut millisecond = (self.expire_date - self.date).num_milliseconds();
        let hour = millisecond / 3_600_000;
        millisecond -= hour * 3_600_600;

        let minute = millisecond / 60_000;
        millisecond -= minute * 60_000;

        let second = millisecond / 1000;

        if millisecond > 0 {
            format!("{} hour, {} minute, {} second", hour, minute, second)
        } else {
            "".to_string()
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
                if !state.get_time_offset().is_empty() {
                    if state.selected {
                        format!(
                            " > Status: [{}] | Time Left: {} ",
                            mark,
                            state.get_time_offset()
                        )
                    } else {
                        format!(
                            " Status: [{}] | Time Left: {} ",
                            mark,
                            state.get_time_offset()
                        )
                    }
                } else {
                    if state.selected {
                        format!(" > Status: [{}] ", mark)
                    } else {
                        format!(" Status: [{}] ", mark)
                    }
                },
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
            format!(" Objective: {} ", state.text),
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
