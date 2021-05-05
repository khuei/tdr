use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::text::Span;
use tui::widgets::{Block, StatefulWidget, Widget};

use crate::theme::style;
use crate::THEME;

pub struct WorkspaceState {
    pub slot: usize,
    pub title: String,
    pub is_selected: bool,
}

impl WorkspaceState {
    pub fn new(slot: usize, title: String) -> WorkspaceState {
        WorkspaceState {
            slot,
            title,
            is_selected: true,
        }
    }
}

pub struct WorkspaceWidget {}

impl StatefulWidget for WorkspaceWidget {
    type State = WorkspaceState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Block::default()
            .title(Span::styled(
                format!("({}) + {: <1000}", state.slot, state.title),
                if state.is_selected {
                    style().fg(THEME.text_dark()).bg(THEME.text_primary())
                } else {
                    style().fg(THEME.text_normal()).bg(THEME.background())
                },
            ))
            .render(area, buf);
    }
}
