use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, panic, thread};

use lazy_static::lazy_static;

use tui::backend::CrosstermBackend;
use tui::style::Color;
use tui::Terminal;

use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::Event;
use crossterm::{cursor, execute, terminal};

mod app;
mod draw;
mod event;
mod query;
mod theme;
mod widget;

lazy_static! {
    pub static ref QUERIES: query::Queries = query::resolve_queries();
    pub static ref REDRAW_REQUEST: (Sender<()>, Receiver<()>) = bounded(1);
    pub static ref THEME: theme::Theme = theme::Theme {
        background: Color::Reset,
        unfinished: Color::Red,
        finished: Color::Green,
        loss: Color::Red,
        text_normal: Color::Reset,
        text_primary: Color::Yellow,
        text_secondary: Color::Cyan,
        text_dark: Color::Black,
        border_primary: Color::Blue,
        border_secondary: Color::Reset,
        border_axis: Color::Blue,
        focused: Color::LightBlue,
        unfocused: Color::DarkGray,
    };
}

fn setup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();

    terminal::disable_raw_mode().unwrap();
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        cleanup_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}

fn setup_ui_events() -> Receiver<Event> {
    let (sender, receiver) = unbounded();
    thread::spawn(move || loop {
        sender.send(crossterm::event::read().unwrap()).unwrap();
    });

    receiver
}

fn main() {
    better_panic::install();

    let queries = QUERIES.clone();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    setup_panic_hook();
    setup_terminal();

    let request_redraw = REDRAW_REQUEST.0.clone();
    let ui_events = setup_ui_events();

    let mut starting_workspaces: Vec<widget::WorkspaceState> = Vec::new();
    let mut starting_items: Vec<Vec<widget::ItemState>> = Vec::new();

    if queries
        .clone()
        .workspace_slot
        .unwrap_or_default()
        .into_iter()
        .count()
        > 0
        && queries
            .clone()
            .item_slot
            .unwrap_or_default()
            .into_iter()
            .count()
            > 0
    {
        for workspace in 0..queries
            .clone()
            .workspace_slot
            .unwrap_or_default()
            .into_iter()
            .count()
        {
            starting_workspaces.push(widget::WorkspaceState::new(
                *queries
                    .clone()
                    .workspace_slot
                    .unwrap_or_default()
                    .get(workspace)
                    .unwrap(),
                queries
                    .clone()
                    .workspace_title
                    .unwrap_or_default()
                    .get(workspace)
                    .unwrap()
                    .to_string(),
                *queries
                    .clone()
                    .workspace_num_of_item
                    .unwrap_or_default()
                    .get(workspace)
                    .unwrap(),
                if workspace == 0 { true } else { false },
            ));

            starting_items.push(Vec::new());

            let mut starting_index: usize = 0;

            for index in 0..workspace {
                starting_index += queries
                    .clone()
                    .workspace_num_of_item
                    .unwrap_or_default()
                    .get(index)
                    .unwrap();
            }

            for item in starting_index
                ..(starting_index
                    + queries
                        .clone()
                        .workspace_num_of_item
                        .unwrap_or_default()
                        .get(workspace)
                        .unwrap())
            {
                starting_items[workspace].push(widget::ItemState::new(
                    *queries
                        .clone()
                        .item_slot
                        .unwrap_or_default()
                        .get(item)
                        .unwrap(),
                    queries
                        .clone()
                        .item_text
                        .unwrap_or_default()
                        .get(item)
                        .unwrap()
                        .to_string(),
                    queries
                        .clone()
                        .item_expire_datetime_string
                        .unwrap_or_default()
                        .get(item)
                        .unwrap()
                        .to_string(),
                    *queries
                        .clone()
                        .item_is_finished
                        .unwrap_or_default()
                        .get(item)
                        .unwrap(),
                    if item == 0 { true } else { false },
                ));
            }
        }
    }

    if starting_workspaces.is_empty() {
        starting_workspaces.push(widget::WorkspaceState::new(
            0,
            "default".to_string(),
            0,
            true,
        ));
        starting_items.push(Vec::new());
    }

    let app = Arc::new(Mutex::new(app::App {
        mode: app::Mode::DisplayItem,
        previous_mode: app::Mode::DisplayItem,
        items: starting_items,
        add_item: widget::AddItemState::new(),
        edit_item: widget::EditItemState::new(),
        current_item: 0,
        workspaces: starting_workspaces,
        add_workspace: widget::AddWorkspaceState::new(),
        edit_workspace: widget::EditWorkspaceState::new(),
        current_workspace: 0,
        is_modified: false,
        help_mode: app::HelpMode::ItemHelp,
        help_item: widget::HelpItemWidget {},
        help_workspace: widget::HelpWorkspaceWidget {},
        summary_scroll_state: Default::default(),
    }));

    let move_app = app.clone();

    thread::spawn(move || {
        let app = move_app;

        let redraw_requested = REDRAW_REQUEST.1.clone();

        loop {
            select! {
                recv(redraw_requested) -> _ => {
                    let mut app = app.lock().unwrap();
                    draw::draw(&mut terminal, &mut app);
                }
                default(Duration::from_millis(500)) => {
                    let mut app = app.lock().unwrap();
                    draw::draw(&mut terminal, &mut app);
                }
            }
        }
    });

    loop {
        select! {
            recv(ui_events) -> message => {
                let mut app = app.lock().unwrap();

                match message.unwrap() {
                    Event::Key(key_event) => {
                        event::handle_key_bindings(app.mode, key_event, &mut app, &request_redraw);
                    }
                    Event::Resize(..) => {
                        let _ = request_redraw.try_send(());
                    }
                    _ => {}
                }
            }
        }
    }
}
