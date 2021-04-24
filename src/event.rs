use app::ScrollDirection;
use crossbeam_channel::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{self, Mode};
use crate::cleanup_terminal;

fn handle_keys_add_item(keycode: KeyCode, mut app: &mut app::App) {
    match keycode {
        KeyCode::Enter => {
            let mut item = app.add_item.enter();

            app.items.push(item);
            app.current_item = app.items.len() - 1;

            app.add_item.reset();
            app.mode = app.previous_mode;
        }
        KeyCode::Char(c) => {
            app.add_item.add_char(c);
        }
        KeyCode::Backspace => {
            app.add_item.del_char();
        }
        KeyCode::Esc => {
            app.add_item.reset();
            if !app.items.is_empty() {
                app.mode = app.previous_mode;
            }
        }
        _ => {}
    }
}

fn handle_keys_display_item(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (keycode, modifiers) {
        (KeyCode::Char('a'), KeyModifiers::NONE) => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::AddItem;
        }
        _ => {}
    }
}

pub fn handle_key_bindings(
    mode: Mode,
    key_event: KeyEvent,
    mut app: &mut app::App,
    request_redraw: &Sender<()>,
) {
    match (mode, key_event.modifiers, key_event.code) {
        (_, KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (mode, KeyModifiers::NONE, KeyCode::Char('q')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (Mode::AddItem, modifiers, keycode) => {
            if modifiers.is_empty() || modifiers == KeyModifiers::SHIFT {
                handle_keys_add_item(keycode, app)
            }
        }
        (Mode::DisplayItem, modifiers, keycode) => {
            handle_keys_display_item(keycode, modifiers, app)
        }
    }

    let _ = request_redraw.try_send(());
}
