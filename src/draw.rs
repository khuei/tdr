use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::{Frame, Terminal};

use crate::app::{App, Mode, ScrollDirection};
use crate::theme::style;
use crate::widget::{block, AddItemWidget, ItemWidget};
use crate::THEME;

pub enum PaddingDirection {
    Top,
    Bottom,
    Left,
    Right,
    All,
}

pub fn add_padding(mut rect: Rect, n: u16, direction: PaddingDirection) -> Rect {
    match direction {
        PaddingDirection::Top => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Bottom => {
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Left => {
            rect.x += n;
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::Right => {
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::All => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n * 2);

            rect.x += n;
            rect.width = rect.width.saturating_sub(n * 2);

            rect
        }
    }
}

fn draw_main<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let mut layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    if let Some(item) = app.items.get_mut(app.current_item) {
        let main_chunks = vec![layout[1]];

        match app.mode {
            Mode::DisplayItem | Mode::AddItem => {
                frame.render_stateful_widget(ItemWidget {}, main_chunks[0], item);
            }
        }
    }
}

fn draw_add_item<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    frame.render_stateful_widget(AddItemWidget {}, area, &mut app.add_item);
}

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    let current_size = terminal.size().unwrap_or_default();

    if current_size.width <= 10 || current_size.height <= 10 {
        return;
    }

    terminal
        .draw(|mut frame| {
            frame.render_widget(Block::default().style(style()), frame.size());

            if app.mode == Mode::AddItem {
                let layout = Layout::default()
                    .constraints(
                        [
                            Constraint::Min(0),
                            Constraint::Length(3),
                            Constraint::Length(5),
                        ]
                        .as_ref(),
                    )
                    .split(frame.size());

                if !app.items.is_empty() {
                    draw_main(&mut frame, app, layout[0]);
                }

                draw_main(&mut frame, app, layout[0]);
                draw_add_item(&mut frame, app, layout[1]);
            } else {
                let layout = frame.size();
                match app.mode {
                    _ => draw_main(&mut frame, app, layout),
                }
            };
        })
        .unwrap();
}
