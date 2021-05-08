use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};
use tui::widgets::{Paragraph, StatefulWidget, Widget, Wrap};

use crate::theme::style;
use crate::THEME;

pub struct AddWorkspaceState {
    input_string: String,
    has_input: bool,
    error_msg: Option<String>,
}

impl AddWorkspaceState {
    pub fn new() -> AddWorkspaceState {
        AddWorkspaceState {
            input_string: String::new(),
            has_input: false,
            error_msg: Some(String::new()),
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.input_string.push(c);
        self.has_input = true;
    }

    pub fn del_char(&mut self) {
        self.input_string.pop();
    }

    pub fn reset(&mut self) {
        self.input_string.drain(..);
        self.has_input = false;
        self.error_msg = None;
    }

    pub fn enter(&mut self, slot: usize, num_of_item: usize) -> super::WorkspaceState {
        super::WorkspaceState::new(slot, self.input_string.clone(), num_of_item, true)
    }
}

pub struct AddWorkspaceWidget {}

impl StatefulWidget for AddWorkspaceWidget {
    type State = AddWorkspaceState;

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
                    &state.input_string,
                    style()
                        .add_modifier(Modifier::BOLD)
                        .fg(THEME.text_secondary),
                ),
            ])
        };
        Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(style().fg(THEME.border_primary))
                    .title(Span::styled(
                        " Add Workspace",
                        style().fg(THEME.text_normal),
                    )),
            )
            .style(style())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
