use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use regex::Regex;
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
    input_datetime: String,
    has_input: bool,
    pub has_expire_datetime: bool,
    error_msg: Option<String>,
}

impl AddItemState {
    pub fn new() -> AddItemState {
        AddItemState {
            input_string: String::new(),
            input_datetime: String::new(),
            has_input: false,
            has_expire_datetime: false,
            error_msg: Some(String::new()),
        }
    }

    pub fn add_char(&mut self, c: char) {
        if self.has_expire_datetime {
            self.input_datetime.push(c);
        } else {
            self.input_string.push(c);
        }
        self.has_input = true;
    }

    pub fn del_char(&mut self) {
        if self.has_expire_datetime {
            self.input_datetime.pop();
        } else {
            self.input_string.pop();
        }
    }

    pub fn reset(&mut self) {
        self.input_datetime.drain(..);
        self.input_string.drain(..);
        self.has_input = false;
        self.error_msg = None;
    }

    pub fn enter(&mut self, slot: usize) -> super::ItemState {
        let input_datetime = if Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(&self.input_datetime)
        {
            format!("{}-04:00 00:00:00", self.input_datetime)
        } else if Regex::new(r"^\d{2}:\d{2}:\d{2}$")
            .unwrap()
            .is_match(&self.input_datetime)
        {
            format!("{} {}", Local::now().date(), self.input_datetime)
        } else if Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$")
            .unwrap()
            .is_match(&self.input_datetime)
        {
            self.input_datetime.replace(" ", "-04:00 ")
        } else {
            "".to_string()
        };

        if !input_datetime.is_empty() {
            let naive_expire_datetime =
                NaiveDateTime::parse_from_str(&input_datetime, "%Y-%m-%d-04:00 %H:%M:%S").unwrap();
            let expire_datetime: DateTime<Local> =
                Local.from_local_datetime(&naive_expire_datetime).unwrap();

            super::ItemState::new(
                slot,
                self.input_string.clone(),
                true,
                expire_datetime,
                if (expire_datetime - Local::now()).num_seconds() > 0 {
                    false
                } else {
                    true
                },
            )
        } else {
            super::ItemState::new(slot, self.input_string.clone(), false, Local::now(), false)
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
                    if state.has_expire_datetime {
                        &state.input_datetime
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
            .block(if state.has_expire_datetime {
                block::new(" Set Expiry Timestamp ")
            } else {
                block::new(" Add Item ")
            })
            .style(style())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
