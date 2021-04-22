use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::{Frame, Terminal};

use crate::app::{App, Mode, ScrollDirection};
use crate::theme::style;
use crate::widget::{block, AddItemWidget};
use crate::THEME;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>) {
    let current_size = terminal.size().unwrap_or_default();

    if current_size.width <= 10 || current_size.height <= 10 {
        return;
    }

    terminal
        .draw(|frame| {
            let chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(100)])
                .split(frame.size());
            let block = Block::default()
                .title(Span::styled(" Todo List ", style().fg(THEME.text_normal())))
                .borders(Borders::ALL)
                .border_style(style().fg(THEME.border_primary()));
            frame.render_widget(block, chunks[0]);
        })
        .unwrap()
}

fn draw_add_item<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    frame.render_stateful_widget(AddItemWidget {}, area, &mut app.add_item);
}
