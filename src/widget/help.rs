use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use super::block;
use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;

const HELP_TEXT: &str = r#"
Quit: q or <Ctrl+c>
Add Item:
  - a: open prompt
  - (while adding):
    - <Ctrl+d>: toggle expire date
    - <Enter>: accept
    - <Escape>: quit
Remove Item: d
Toggle Item: y
Scroll Pane:
  - j: down
  - k: up
"#;

pub const HELP_WIDTH: usize = 38;
pub const HELP_HEIGHT: usize = 16;

#[derive(Copy, Clone)]
pub struct HelpWidget {}

impl HelpWidget {
    pub fn get_rect(self, area: Rect) -> Rect {
        Rect {
            x: (area.width - HELP_WIDTH as u16) / 2,
            y: (area.height - HELP_HEIGHT as u16) / 2,
            width: HELP_WIDTH as u16,
            height: HELP_HEIGHT as u16,
        }
    }
}

impl Widget for HelpWidget {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        block::new(" Help ").render(area, buf);
        area = add_padding(area, 1, PaddingDirection::All);
        area = add_padding(area, 1, PaddingDirection::Left);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(HELP_WIDTH as u16)])
            .split(area);

        let help_text: Vec<_> = HELP_TEXT
            .lines()
            .map(|line| {
                Spans::from(Span::styled(
                    format!("{}\n", line),
                    style().fg(THEME.text_normal()),
                ))
            })
            .collect();

        Paragraph::new(help_text).render(layout[0], buf);
    }
}
