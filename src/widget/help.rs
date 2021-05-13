use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};
use tui::widgets::{Paragraph, Widget};

use crate::draw::{add_padding, PaddingDirection};
use crate::theme::style;
use crate::THEME;

const LEFT_TEXT: &str = r#"
Item Display:
- <w>: add workspace
- <->: display workspace
- <a>: open add item prompt
- <e>: edit item
- <d>: remove item
- <space>: toggle item
- <r>: rename current workspace
- <J>: scroll workspace down
- <K>: scroll workspace up
- <j>: scroll down
- <k>: scroll up
- <q> or <Ctrl+c>: quit
- <?>: toggle help display

Add Item:
- <Ctrl+d>: toggle input
            timestamp
- <Enter>: accept
- <Escape>: exit
"#;

const RIGHT_TEXT: &str = r#"
Workspace Display:
- <enter>: select workspace
- <e>: edit workspace
- <d>: remove workspace
- <j>: scroll down
- <k>: scroll up

Add Workspace:
- <Enter>: accept
- <Escape>: exit
"#;

const LEFT_WIDTH: usize = 32;
const RIGHT_WIDTH: usize = 27;
pub const HELP_WIDTH: usize = 2 + LEFT_WIDTH + 2 + RIGHT_WIDTH + 2;
pub const HELP_HEIGHT: usize = 21;

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
        Block::default()
            .borders(Borders::ALL)
            .border_style(style().fg(THEME.border_primary))
            .title(Span::styled(" Help ", style().fg(THEME.text_normal)))
            .render(area, buf);
        area = add_padding(area, 1, PaddingDirection::All);
        area = add_padding(area, 1, PaddingDirection::Left);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(LEFT_WIDTH as u16),
                Constraint::Length(2),
                Constraint::Length(RIGHT_WIDTH as u16),
            ])
            .split(area);

        let left_text: Vec<_> = LEFT_TEXT
            .lines()
            .map(|line| {
                Spans::from(Span::styled(
                    format!("{}\n", line),
                    style().fg(THEME.text_normal),
                ))
            })
            .collect();

        let right_text: Vec<_> = RIGHT_TEXT
            .lines()
            .map(|line| {
                Spans::from(Span::styled(
                    format!("{}\n", line),
                    style().fg(THEME.text_normal),
                ))
            })
            .collect();

        Paragraph::new(left_text).render(layout[0], buf);
        Paragraph::new(right_text).render(layout[2], buf);
    }
}
