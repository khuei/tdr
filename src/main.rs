use std::time::Duration;
use std::{io, panic, thread};

use lazy_static::lazy_static;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::{cursor, execute, terminal};

mod app;
mod draw;
mod opts;
mod theme;
mod widget;

lazy_static! {
    pub static ref OPTS: opts::Opts = opts::resolve_opts();
    pub static ref REDRAW_REQUEST: (Sender<()>, Receiver<()>) = bounded(1);
    pub static ref THEME: theme::Theme = OPTS.theme.unwrap_or_default();
}

fn setup_terminal() {
    execute!(io::stdout(), cursor::Hide).unwrap();
    execute!(io::stdout(), terminal::EnterAlternateScreen).unwrap();

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();
    execute!(io::stdout(), cursor::Show).unwrap();

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

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    setup_panic_hook();
    setup_terminal();

    let request_redraw = REDRAW_REQUEST.0.clone();

    thread::spawn(move || {
        let redraw_requested = REDRAW_REQUEST.1.clone();

        loop {
            select! {
                recv(redraw_requested) -> _ => {
                    draw::draw(&mut terminal);
                }
                default(Duration::from_millis(500)) => {
                    draw::draw(&mut terminal);
                }
            }
        }
    });

    loop {
        select! {
            recv(setup_ui_events()) -> message => {
                match message.unwrap() {
                    Event::Key(key_event) => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            match key_event.code {
                                KeyCode::Char('c') => {
                                    break
                                },
                                _ => {}
                            }
                        }
                    }
                    Event::Resize(..) => {
                        let _ = request_redraw.try_send(());
                    }
                    _ => {}
                }
            }
        }
    }

    cleanup_terminal()
}
