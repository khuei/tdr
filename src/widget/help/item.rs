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
Item Display:
<k>               Scroll up
<j>               Scroll down
<w>               Create new workspace
<->               Open workspace window
<a>               Open add item window
<e>               Open edit item window
<d>               Delete selected item
<Space>           Toggle item's completion status
<x>               Toggle item's late status
<K>               Scroll workspace up
<J>               Scroll workspace down
<q>               Rename current workspace
<q>, <Ctrl-c>     quit program

Add Item Display:
<Ctrl-d>          Toggle input timestamp window
<Enter>           Create item
<Escape>          Exit window
"#;

#[derive(Copy, Clone)]
pub struct HelpItemWidget {}

impl HelpItemWidget {
    pub fn get_rect(self, area: Rect) -> Rect {
        Rect {
            x: (area.width - HELP_WIDTH as u16) / 2,
            y: (area.height - HELP_HEIGHT as u16) / 2,
            width: HELP_WIDTH as u16,
            height: HELP_HEIGHT as u16,
        }
    }
}

impl Widget for HelpItemWidget {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(style().fg(THEME.border_primary))
            .title(Span::styled(
                " Help - Item (<j>: next, <k>: previous) ",
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
