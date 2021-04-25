use app::ScrollDirection;
use crossbeam_channel::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{self, Mode};
use crate::cleanup_terminal;

fn handle_keys_add_item(keycode: KeyCode, mut app: &mut app::App) {
    match keycode {
        KeyCode::Enter => {
            let item = app.add_item.enter(app.items.len());

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
            app.mode = app.previous_mode;
        }
        _ => {}
    }
}

fn handle_keys_display_item(keycode: KeyCode, _modifiers: KeyModifiers, mut app: &mut app::App) {
    match keycode {
        KeyCode::Char('j') => {
            if !app.items.is_empty() {
                if app.current_item == app.items.len() - 1 {
                    app.current_item = 0;
                } else {
                    app.current_item += 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Down);
            }
        }
        KeyCode::Char('k') => {
            if !app.items.is_empty() {
                if app.current_item == 0 {
                    app.current_item = app.items.len() - 1;
                } else {
                    app.current_item -= 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Up);
            }
        }
        KeyCode::Char('a') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::AddItem;
        }
        KeyCode::Char('y') => {
            let item = app.items.get_mut(app.current_item).unwrap();
            if item.done {
                item.done = false;
            } else {
                item.done = true;
            }
        }
        KeyCode::Char('d') => {
            app.items.remove(app.current_item);

            if app.current_item != 0 {
                app.current_item -= 1;
            }
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
        (Mode::DisplayHelp, modifiers, keycode) => {
            if modifiers.is_empty() && (matches!(keycode, KeyCode::Esc | KeyCode::Char('?'))) {
                app.mode = app.previous_mode;
            }
        }
        (_, KeyModifiers::NONE, KeyCode::Char('?')) => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::DisplayHelp;
        }
        (Mode::AddItem, modifiers, keycode) => {
            if modifiers.is_empty() || modifiers == KeyModifiers::SHIFT {
                handle_keys_add_item(keycode, app)
            }
        }
        (_, KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (_, KeyModifiers::NONE, KeyCode::Char('q')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (Mode::DisplayItem, modifiers, keycode) => {
            handle_keys_display_item(keycode, modifiers, app)
        }
    }

    let _ = request_redraw.try_send(());
}
