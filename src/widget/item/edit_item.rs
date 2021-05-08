use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};
use tui::widgets::{Paragraph, StatefulWidget, Widget, Wrap};

use crate::theme::style;
use crate::THEME;

pub struct EditItemState {
    pub input_string: String,
    pub input_datetime: String,
    has_input: bool,
    pub has_expire_datetime: bool,
    error_msg: Option<String>,
}

impl EditItemState {
    pub fn new() -> EditItemState {
        EditItemState {
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

    pub fn enter(&mut self, slot: usize, workspace: String) -> super::ItemState {
        super::ItemState::new(
            slot,
            workspace,
            self.input_string.clone(),
            self.input_datetime.clone(),
            false,
            true,
        )
    }
}

pub struct EditItemWidget {}

impl StatefulWidget for EditItemWidget {
    type State = EditItemState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let spans = if !state.has_input && state.error_msg.is_some() {
            Spans::from(vec![
                Span::styled("> ", style().fg(THEME.text_normal)),
                Span::styled(
                    state.error_msg.as_ref().unwrap(),
                    style().add_modifier(Modifier::BOLD).fg(THEME.loss),
                ),
            ])
        } else {
            Spans::from(vec![
                Span::styled("> ", style().fg(THEME.text_normal)),
                Span::styled(
                    if state.has_expire_datetime {
                        &state.input_datetime
                    } else {
                        &state.input_string
                    },
                    style()
                        .add_modifier(Modifier::BOLD)
                        .fg(THEME.text_secondary),
                ),
            ])
        };
        Paragraph::new(spans)
            .block(if state.has_expire_datetime {
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(style().fg(THEME.border_primary))
                    .title(Span::styled(
                        " Modify Expiry Timestamp ",
                        style().fg(THEME.text_normal),
                    ))
            } else {
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(style().fg(THEME.border_primary))
                    .title(Span::styled(" Modify Item ", style().fg(THEME.text_normal)))
            })
            .style(style())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
