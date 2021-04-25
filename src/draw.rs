use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::{Frame, Terminal};

use crate::app::{App, Mode, ScrollDirection};
use crate::theme::style;
use crate::widget::{block, AddItemWidget, ItemWidget, HELP_HEIGHT, HELP_WIDTH};
use crate::THEME;

#[allow(dead_code)]
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

fn draw_add_item<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    frame.render_stateful_widget(AddItemWidget {}, area, &mut app.add_item);
}

fn draw_help<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let mut layout = area;

    if layout.width < HELP_WIDTH as u16 || layout.height < HELP_HEIGHT as u16 {
        frame.render_widget(
            Paragraph::new(Text::styled(
                "Increase screen size to display help",
                style(),
            )),
            layout,
        );
    } else {
        layout = app.help.get_rect(layout);
        frame.render_widget(app.help, layout)
    }
}

fn draw_main<B: Backend>(frame: &mut Frame<B>, app: &mut App, mut area: Rect) {
    let border = block::new(" List ");
    frame.render_widget(border, area);
    area = add_padding(area, 1, PaddingDirection::All);

    let item_widget_height = 3;
    let height = area.height;
    let num_to_render = (((height - 3) / item_widget_height) as usize).min(app.items.len());

    let mut scroll_offset = if let Some(direction) = app.summary_scroll_state.queued_scroll.take() {
        let new_offset = match direction {
            ScrollDirection::Up => {
                if app.summary_scroll_state.offset == 0 {
                    0
                } else {
                    (app.summary_scroll_state.offset - 1).min(app.items.len())
                }
            }
            ScrollDirection::Down => {
                (app.summary_scroll_state.offset + 1).min(app.items.len() - num_to_render)
            }
        };

        app.summary_scroll_state.offset = new_offset;

        new_offset
    } else {
        app.summary_scroll_state.offset
    };

    if num_to_render + scroll_offset > app.items.len() {
        scroll_offset -= (num_to_render + scroll_offset) - app.items.len();
        app.summary_scroll_state.offset = scroll_offset;
    }

    let mut layout = Layout::default()
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length((num_to_render * item_widget_height as usize) as u16),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);

    let constraints = app.items[scroll_offset..num_to_render + scroll_offset]
        .iter()
        .map(|_| Constraint::Length(item_widget_height))
        .collect::<Vec<_>>();

    let item_layout = Layout::default().constraints(constraints).split(layout[1]);

    for (idx, item) in app.items[scroll_offset..num_to_render + scroll_offset]
        .iter_mut()
        .enumerate()
    {
        frame.render_stateful_widget(ItemWidget {}, item_layout[idx], item);
    }

    layout[2] = add_padding(layout[2], 1, PaddingDirection::Left);
    frame.render_widget(Clear, layout[2]);
    frame.render_widget(Block::default().style(style()), layout[2]);

    let offset = layout[2].height - 2;
    layout[2] = add_padding(layout[2], offset, PaddingDirection::Top);

    frame.render_widget(
        Block::default()
            .borders(Borders::TOP)
            .border_style(style().fg(THEME.border_secondary())),
        layout[2],
    );

    layout[2] = add_padding(layout[2], 1, PaddingDirection::Top);

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(layout[2]);

    frame.render_widget(
        Paragraph::new(format!("current item: {}", app.current_item)),
        bottom_layout[0],
    );

    let more_up = scroll_offset > 0;
    let more_down = scroll_offset + num_to_render < app.items.len();

    let up_arrow = Span::styled(
        "↑",
        style().fg(if more_up {
            THEME.highlight_focused()
        } else {
            THEME.highlight_unfocused()
        }),
    );
    let down_arrow = Span::styled(
        "↓",
        style().fg(if more_down {
            THEME.highlight_focused()
        } else {
            THEME.highlight_unfocused()
        }),
    );

    frame.render_widget(
        Paragraph::new(Spans::from(vec![up_arrow, Span::raw(" "), down_arrow])),
        bottom_layout[1],
    );
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
                    .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                    .split(frame.size());

                if !app.items.is_empty() {
                    draw_main(&mut frame, app, layout[0]);
                }

                draw_main(&mut frame, app, layout[0]);
                draw_add_item(&mut frame, app, layout[1]);
            } else {
                let layout = frame.size();
                match app.mode {
                    Mode::DisplayHelp => draw_help(&mut frame, app, layout),
                    _ => draw_main(&mut frame, app, layout),
                }
            };
        })
        .unwrap();
}
