use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use regex::Regex;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;

pub struct ItemState {
    pub slot: usize,
    pub workspace: String,
    pub text: String,
    pub has_expire_datetime: bool,
    pub expire_datetime_string: String,
    pub expire_datetime: DateTime<Local>,
    pub is_finished: bool,
    pub is_late: bool,
    pub is_selected: bool,
}

impl ItemState {
    pub fn new(
        slot: usize,
        workspace: String,
        text: String,
        expire_datetime_string: String,
        is_finished: bool,
        is_selected: bool,
    ) -> ItemState {
        let input_datetime = if Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(&expire_datetime_string)
        {
            format!("{}-04:00 00:00:00", expire_datetime_string)
        } else if Regex::new(r"^\d{2}:\d{2}:\d{2}$")
            .unwrap()
            .is_match(&expire_datetime_string)
        {
            format!("{} {}", Local::now().date(), expire_datetime_string)
        } else if Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$")
            .unwrap()
            .is_match(&expire_datetime_string)
        {
            expire_datetime_string.replace(" ", "-04:00 ")
        } else {
            "".to_string()
        };

        let has_expire_datetime = {
            if !input_datetime.is_empty() {
                true
            } else {
                false
            }
        };

        let expire_datetime: DateTime<Local> = {
            if has_expire_datetime {
                Local
                    .from_local_datetime(
                        &NaiveDateTime::parse_from_str(&input_datetime, "%Y-%m-%d-04:00 %H:%M:%S")
                            .unwrap(),
                    )
                    .unwrap()
            } else {
                Local::now()
            }
        };

        let is_late = {
            if !input_datetime.is_empty() {
                if (expire_datetime - Local::now()).num_seconds() > 0 {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        };

        ItemState {
            slot,
            workspace,
            text,
            has_expire_datetime,
            expire_datetime_string,
            expire_datetime,
            is_finished,
            is_late,
            is_selected,
        }
    }

    fn get_time_offset(&mut self) -> String {
        let offset = self.expire_datetime - Local::now();
        let mut second = offset.num_seconds();

        if offset.num_seconds() < 0 {
            self.is_late = true;
        }

        let minute: i64;
        let hour: i64;
        let day: i64;
        let week: i64;

        let mut output: String = String::new();

        if offset.num_weeks().abs() > 0 {
            week = second / 604800;
            second -= week * 604800;
            output.push_str(&week.to_string());
            output.push_str(" week, ");
        }

        if offset.num_days().abs() > 0 {
            day = second / 86400;
            second -= day * 86400;
            output.push_str(&day.to_string());
            output.push_str(" day, ");
        }

        if offset.num_hours().abs() > 0 {
            hour = second / 3600;
            second -= hour * 3600;
            output.push_str(&hour.to_string());
            output.push_str(" hour, ");
        }

        if offset.num_minutes().abs() > 0 {
            minute = second / 60;
            second -= minute * 60;
            output.push_str(&minute.to_string());
            output.push_str(" minute, ");
        }

        output.push_str(&second.to_string());
        output.push_str(" second ");

        output
    }
}

pub struct ItemWidget {}

impl StatefulWidget for ItemWidget {
    type State = ItemState;

    fn render(self, mut area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mark = if state.is_finished { "✓" } else { "x" };

        Block::default()
            .title(Span::styled(
                if state.has_expire_datetime {
                    if state.is_selected {
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
                    if state.is_selected {
                        format!(" > Status: [{}] ", mark)
                    } else {
                        format!(" Status: [{}] ", mark)
                    }
                },
                if state.is_finished {
                    style().fg(THEME.finished())
                } else if state.is_late {
                    style().fg(THEME.loss())
                } else {
                    style().fg(THEME.text_normal())
                },
            ))
            .borders(Borders::ALL)
            .border_style(if state.is_finished {
                style().fg(THEME.finished())
            } else if state.is_late {
                style().fg(THEME.loss())
            } else {
                style().fg(THEME.border_secondary())
            })
            .render(area, buf);
        area = add_padding(area, 1, PaddingDirection::Top);
        area = add_padding(area, 1, PaddingDirection::Left);
        area = add_padding(area, 1, PaddingDirection::Right);

        let text = vec![Span::styled(
            format!(" Objective: {} ", state.text),
            if state.is_finished {
                style().fg(THEME.finished())
            } else if state.is_late {
                style().fg(THEME.loss())
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
