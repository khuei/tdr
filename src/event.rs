use std::fs;

use app::ScrollDirection;
use crossbeam_channel::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{self, Mode};
use crate::cleanup_terminal;

use anyhow::{format_err, Error};

fn write_on_exit(app: &mut app::App) -> Result<(), Error> {
    let config_dir = dirs_next::config_dir()
        .ok_or_else(|| format_err!("Could not get config directory"))?
        .join("todo-rs");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    let query_path = config_dir.join("query.yml");

    let mut query_text: String = String::from("");

    query_text.push_str("slot:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.slot));
    }

    query_text.push_str("text:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.text));
    }

    query_text.push_str("expire_datetime_string:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.expire_datetime_string));
    }

    query_text.push_str("is_finished:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.is_finished));
    }

    query_text.push_str("is_selected:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.is_selected));
    }

    let _ = fs::write(&query_path, query_text);

    Ok(())
}

fn handle_keys_add_item(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (modifiers, keycode) {
        (KeyModifiers::NONE, KeyCode::Enter) => {
            app.add_item.has_expire_datetime = false;
            let item = app.add_item.enter(app.items.len());

            app.items.push(item);
            app.current_item = app.items.len() - 1;

            app.add_item.reset();
            app.mode = app.previous_mode;
        }
        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
            if app.add_item.has_expire_datetime {
                app.add_item.has_expire_datetime = false;
            } else {
                app.add_item.has_expire_datetime = true;
            }
        }
        (KeyModifiers::NONE, KeyCode::Char(c)) => {
            app.add_item.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            app.add_item.del_char();
        }
        (KeyModifiers::NONE, KeyCode::Esc) => {
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
                    app.current_item = app.current_item;
                } else {
                    app.current_item += 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Down);
            }
        }
        KeyCode::Char('k') => {
            if !app.items.is_empty() {
                if app.current_item == 0 {
                    app.current_item = app.current_item;
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
            if item.is_finished {
                item.is_finished = false;
            } else {
                item.is_finished = true;
            }
        }
        KeyCode::Char('d') => {
            app.items.remove(app.current_item);

            if !app.items.is_empty() {
                for item in app.items.iter_mut() {
                    if item.slot > app.current_item {
                        item.slot -= 1;
                    }
                }
            }

            if app.current_item != 0 {
                app.current_item -= 1;
            }
        }
        KeyCode::Char('?') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::DisplayHelp;
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
        (Mode::AddItem, modifiers, keycode) => handle_keys_add_item(keycode, modifiers, app),
        (_, KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            write_on_exit(app).expect("could not store content");
            cleanup_terminal();
            std::process::exit(0);
        }
        (_, KeyModifiers::NONE, KeyCode::Char('q')) => {
            write_on_exit(app).expect("could not store content");
            cleanup_terminal();
            std::process::exit(0);
        }
        (Mode::DisplayItem, modifiers, keycode) => {
            handle_keys_display_item(keycode, modifiers, app)
        }
    }

    if !app.items.is_empty() {
        for item in app.items.iter_mut() {
            if item.slot == app.current_item {
                item.is_selected = true;
            } else {
                item.is_selected = false;
            }
        }
    }

    let _ = request_redraw.try_send(());
}
