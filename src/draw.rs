use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>) {
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .split(frame.size());
        let block = Block::default()
            .title(" Todo List ")
            .borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);
    }).unwrap()
}
