use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, panic, thread};

use lazy_static::lazy_static;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::Event;
use crossterm::{cursor, execute, terminal};

mod app;
mod draw;
mod event;
mod opts;
mod query;
mod theme;
mod widget;

lazy_static! {
    pub static ref OPTS: opts::Opts = opts::resolve_opts();
    pub static ref QUERIES: query::Queries = query::resolve_queries();
    pub static ref REDRAW_REQUEST: (Sender<()>, Receiver<()>) = bounded(1);
    pub static ref THEME: theme::Theme = OPTS.theme.unwrap_or_default();
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

    let mut starting_items: Vec<widget::ItemState> = Vec::new();

    for index in 0..queries.clone().slot.unwrap_or_default().into_iter().count() {
        if queries.clone().slot.unwrap_or_default().into_iter().count() > 0 {
            starting_items.push(widget::ItemState::new(
                *queries
                    .clone()
                    .slot
                    .unwrap_or_default()
                    .get_mut(index)
                    .unwrap(),
                queries
                    .clone()
                    .text
                    .unwrap_or_default()
                    .get_mut(index)
                    .unwrap()
                    .to_string(),
                queries
                    .clone()
                    .expire_datetime_string
                    .unwrap_or_default()
                    .get_mut(index)
                    .unwrap()
                    .to_string(),
            ));
        }
    }

    let app = Arc::new(Mutex::new(app::App {
        mode: app::Mode::DisplayItem,
        previous_mode: app::Mode::DisplayItem,
        items: starting_items,
        add_item: widget::AddItemState::new(),
        current_item: 0,
        help: widget::HelpWidget {},
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
