use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};
use tui::widgets::{Paragraph, Widget};

use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;

use super::HELP_HEIGHT;
use super::HELP_WIDTH;

const TEXT: &str = r#"
Workspace Display:
<j>          Scroll down
<k>          Scroll up
<enter>      Enter selected workspace
<e>          Edit name of selected workspace
<d>          Delete selected workspace

Add Workspace:
<Enter>      Create workspace
<Escape>     Exit window
"#;

#[derive(Copy, Clone)]
pub struct HelpWorkspaceWidget {}

impl HelpWorkspaceWidget {
    pub fn get_rect(self, area: Rect) -> Rect {
        Rect {
            x: (area.width - HELP_WIDTH as u16) / 2,
            y: (area.height - HELP_HEIGHT as u16) / 2,
            width: HELP_WIDTH as u16,
            height: HELP_HEIGHT as u16,
        }
    }
}

impl Widget for HelpWorkspaceWidget {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(style().fg(THEME.border_primary))
            .title(Span::styled(
                " Help - Workspace (<j>: next, <k>: previous) ",
                style().fg(THEME.text_normal),
            ))
            .render(area, buf);
        area = add_padding(area, 1, PaddingDirection::All);
        area = add_padding(area, 1, PaddingDirection::Left);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(59 as u16),
                Constraint::Length(2),
            ])
            .split(area);

        let text: Vec<_> = TEXT
            .lines()
            .map(|line| {
                Spans::from(Span::styled(
                    format!("{}", line),
                    style().fg(THEME.text_normal),
                ))
            })
            .collect();

        Paragraph::new(text).render(layout[1], buf);
    }
}
