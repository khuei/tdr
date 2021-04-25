use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, StatefulWidget, Widget, Wrap};

use super::block;
use crate::theme::style;
use crate::THEME;

pub struct AddItemState {
    input_string: String,
    input_date: String,
    has_input: bool,
    pub has_expire_date: bool,
    error_msg: Option<String>,
}

impl AddItemState {
    pub fn new() -> AddItemState {
        AddItemState {
            input_string: String::new(),
            input_date: String::new(),
            has_input: false,
            has_expire_date: false,
            error_msg: Some(String::new()),
        }
    }

    pub fn add_char(&mut self, c: char) {
        if self.has_expire_date {
            self.input_date.push(c);
        } else {
            self.input_string.push(c);
        }
        self.has_input = true;
    }

    pub fn del_char(&mut self) {
        if self.has_expire_date {
            self.input_date.pop();
        } else {
            self.input_string.pop();
        }
    }

    pub fn reset(&mut self) {
        if self.has_expire_date {
            self.input_date.drain(..);
        } else {
            self.input_string.drain(..);
        }
        self.has_input = false;
        self.error_msg = None;
    }

    pub fn enter(&mut self, slot: usize) -> super::ItemState {
        if !self.input_date.is_empty() {
            let naive_expire_date =
                NaiveDateTime::parse_from_str(&self.input_date, "%Y-%m-%d %H:%M:%S").unwrap();
            let expire_date: DateTime<Local> =
                Local.from_local_datetime(&naive_expire_date).unwrap();
            super::ItemState::new(slot, self.input_string.clone(), Local::now(), expire_date)
        } else {
            super::ItemState::new(slot, self.input_string.clone(), Local::now(), Local::now())
        }
    }
}

pub struct AddItemWidget {}

impl StatefulWidget for AddItemWidget {
    type State = AddItemState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let spans = if !state.has_input && state.error_msg.is_some() {
            Spans::from(vec![
                Span::styled("> ", style().fg(THEME.text_normal())),
                Span::styled(
                    state.error_msg.as_ref().unwrap(),
                    style().add_modifier(Modifier::BOLD).fg(THEME.loss()),
                ),
            ])
        } else {
            Spans::from(vec![
                Span::styled("> ", style().fg(THEME.text_normal())),
                Span::styled(
                    if state.has_expire_date {
                        &state.input_date
                    } else {
                        &state.input_string
                    },
                    style()
                        .add_modifier(Modifier::BOLD)
                        .fg(THEME.text_secondary()),
                ),
            ])
        };
        Paragraph::new(spans)
            .block(if state.has_expire_date {
                block::new(" Set Expiry Date ")
            } else {
                block::new(" Add Item ")
            })
            .style(style())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
