use std::{io, panic, thread};

use tui::backend::CrosstermBackend;
use tui::Terminal;

use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::{cursor, execute, terminal};

mod draw;

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

    draw::draw(&mut terminal);

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
                    _ => {}
                }
            }
        }
    };

    cleanup_terminal()
}
