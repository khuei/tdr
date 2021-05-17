use std::fs;

use app::ScrollDirection;
use crossbeam_channel::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{self, Mode};
use crate::cleanup_terminal;

use anyhow::{format_err, Error};

fn write_on_exit(app: &mut app::App) -> Result<(), Error> {
    let query_path = dirs_next::home_dir()
        .ok_or_else(|| format_err!("could not get home directory"))?
        .join(".todo.yml");

    let mut query_text: String = String::from("");

    query_text.push_str("workspace_slot:\n");
    for workspace in app.workspaces.iter() {
        query_text.push_str(&format!("    - {}\n", workspace.slot));
    }

    query_text.push_str("workspace_title:\n");
    for workspace in app.workspaces.iter() {
        query_text.push_str(&format!("    - {}\n", workspace.title));
    }

    query_text.push_str("workspace_num_of_item:\n");
    for workspace in app.workspaces.iter() {
        query_text.push_str(&format!("    - {}\n", workspace.num_of_item));
    }

    query_text.push_str("workspace_is_selected:\n");
    for workspace in app.workspaces.iter() {
        query_text.push_str(&format!("    - {}\n", workspace.is_selected));
    }

    query_text.push_str("item_slot:\n");
    for item in app.items.iter() {
        query_text.push_str(&format!("    - {}\n", item.slot));
    }

    query_text.push_str("item_workspace:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.workspace));
    }

    query_text.push_str("item_text:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.text));
    }

    query_text.push_str("item_expire_datetime_string:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.expire_datetime_string));
    }

    query_text.push_str("item_is_finished:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.is_finished));
    }

    query_text.push_str("item_is_selected:\n");
    for item in app.items.iter_mut() {
        query_text.push_str(&format!("    - {}\n", item.is_selected));
    }

    let _ = fs::write(&query_path, query_text);

    Ok(())
}

fn handle_keys_add_workspace(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (modifiers, keycode) {
        (KeyModifiers::NONE, KeyCode::Enter) => {
            if !app.add_workspace.input_string.is_empty() {
                let workspace = app.add_workspace.enter(
                    app.workspaces.len(),
                    app.items
                        .iter()
                        .filter(|i| i.workspace == app.workspaces[app.current_workspace].title)
                        .count(),
                );

                app.workspaces.push(workspace);
                app.current_workspace = app.workspaces.len() - 1;

                app.add_workspace.reset();
                app.mode = app.previous_mode;
            } else {
                app.mode = app.previous_mode;
            }
        }
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            app.add_workspace.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Char(c)) => {
            app.add_workspace.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            app.add_workspace.del_char();
        }
        (KeyModifiers::NONE, KeyCode::Esc) => {
            app.add_workspace.reset();
            app.mode = app.previous_mode;
        }
        _ => {}
    }
}

fn handle_keys_edit_workspace(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (modifiers, keycode) {
        (KeyModifiers::NONE, KeyCode::Enter) => {
            let current_title = app.workspaces[app.current_workspace].title.clone();

            if app.edit_workspace.input_string.is_empty() {
                app.edit_workspace.input_string = current_title.clone();
            }

            for item in app
                .items
                .iter_mut()
                .filter(|i| i.workspace == current_title)
            {
                item.workspace = app.edit_workspace.input_string.clone();
            }

            let workspace = app.edit_workspace.enter(
                app.current_workspace,
                app.workspaces[app.current_workspace].num_of_item,
            );

            app.workspaces[app.current_workspace] = workspace;

            app.edit_workspace.reset();
            app.mode = app.previous_mode;
        }
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            app.edit_workspace.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Char(c)) => {
            app.edit_workspace.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            app.edit_workspace.del_char();
        }
        (KeyModifiers::NONE, KeyCode::Esc) => {
            app.edit_workspace.reset();
            app.mode = app.previous_mode;
        }
        _ => {}
    }
}

fn handle_keys_display_workspace(
    keycode: KeyCode,
    _modifiers: KeyModifiers,
    mut app: &mut app::App,
) {
    match keycode {
        KeyCode::Enter => {
            if app.workspaces.is_empty() {
                app.add_workspace.input_string = "default".to_string();
                app.workspaces.push(app.add_workspace.enter(0, 0));
            }

            app.current_item = 0;
            app.previous_mode = app.mode;
            app.mode = app::Mode::DisplayItem;
        }
        KeyCode::Char('j') => {
            if !app.workspaces.is_empty() {
                if app.current_workspace == app.workspaces.len() - 1 {
                    app.current_workspace = app.current_workspace;
                } else {
                    app.current_workspace += 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Down);
            }
        }
        KeyCode::Char('k') => {
            if !app.workspaces.is_empty() {
                if app.current_workspace == 0 {
                    app.current_workspace = app.current_workspace;
                } else {
                    app.current_workspace -= 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Up);
            }
        }
        KeyCode::Char('e') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::EditWorkspace;
        }
        KeyCode::Char('d') => {
            let target = app.workspaces[app.current_workspace].title.clone();
            app.items.retain(|i| i.workspace != target);

            if !app.workspaces.is_empty() {
                app.workspaces.remove(app.current_workspace);
            }

            if !app.workspaces.is_empty() {
                for workspace in app.workspaces.iter_mut() {
                    if workspace.slot > app.current_workspace {
                        workspace.slot -= 1;
                    }
                }
            }

            if app.current_workspace != 0 {
                app.current_workspace -= 1;
            }
        }
        KeyCode::Char('?') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::DisplayHelp;
        }
        _ => {}
    }
}

fn handle_keys_add_item(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (modifiers, keycode) {
        (KeyModifiers::NONE, KeyCode::Enter) => {
            app.add_item.has_expire_datetime = false;

            if !app.add_item.input_string.is_empty() {
                let item = app.add_item.enter(
                    app.workspaces[app.current_workspace].num_of_item,
                    app.workspaces[app.current_workspace].title.clone(),
                );

                app.items.push(item);
                app.current_item = app.workspaces[app.current_workspace].num_of_item;

                app.add_item.reset();
                app.mode = app.previous_mode;
            } else {
                app.mode = app.previous_mode;
            }
        }
        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
            if app.add_item.has_expire_datetime {
                app.add_item.has_expire_datetime = false;
            } else {
                app.add_item.has_expire_datetime = true;
            }
        }
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            app.add_item.add_char(c);
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

fn handle_keys_edit_item(keycode: KeyCode, modifiers: KeyModifiers, mut app: &mut app::App) {
    match (modifiers, keycode) {
        (KeyModifiers::NONE, KeyCode::Enter) => {
            app.edit_item.has_expire_datetime = false;

            if app.edit_item.input_string.is_empty() {
                app.edit_item.input_string =
                    app.items.get_mut(app.current_item).unwrap().text.clone();
            }

            if app.edit_item.input_datetime.is_empty() {
                app.edit_item.input_datetime = app
                    .items
                    .get_mut(app.current_item)
                    .unwrap()
                    .expire_datetime_string
                    .clone();
            }

            let item = app.edit_item.enter(
                app.current_item,
                app.workspaces
                    .get_mut(app.current_workspace)
                    .unwrap()
                    .title
                    .clone(),
            );

            let mut index: usize = 0;

            if app.current_workspace != 0 {
                for workspace in app.workspaces[0..app.current_workspace].iter() {
                    index += workspace.num_of_item;
                }
            }

            index += app.current_item;

            app.items[index] = item;

            app.edit_item.reset();
            app.mode = app.previous_mode;
        }
        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
            if app.edit_item.has_expire_datetime {
                app.edit_item.has_expire_datetime = false;
            } else {
                app.edit_item.has_expire_datetime = true;
            }
        }
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            app.edit_item.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Char(c)) => {
            app.edit_item.add_char(c);
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => {
            app.edit_item.del_char();
        }
        (KeyModifiers::NONE, KeyCode::Esc) => {
            app.edit_item.reset();
            app.mode = app.previous_mode;
        }
        _ => {}
    }
}

fn handle_keys_display_item(keycode: KeyCode, _modifiers: KeyModifiers, mut app: &mut app::App) {
    match keycode {
        KeyCode::Char('J') => {
            app.current_item = 0;
            if !app.workspaces.is_empty() {
                if app.current_workspace == app.workspaces.len() - 1 {
                    app.current_workspace = app.current_workspace;
                } else {
                    app.current_workspace += 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Down);
            }
        }
        KeyCode::Char('K') => {
            app.current_item = 0;
            if !app.workspaces.is_empty() {
                if app.current_workspace == 0 {
                    app.current_workspace = app.current_workspace;
                } else {
                    app.current_workspace -= 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Up);
            }
        }
        KeyCode::Char('j') => {
            if !app.items.is_empty() && app.workspaces[app.current_workspace].num_of_item != 0 {
                if app.current_item == app.workspaces[app.current_workspace].num_of_item - 1 {
                    app.current_item = app.current_item;
                } else {
                    app.current_item += 1;
                }
                app.summary_scroll_state.queued_scroll = Some(ScrollDirection::Down);
            }
        }
        KeyCode::Char('k') => {
            if !app.items.is_empty() && app.workspaces[app.current_workspace].num_of_item != 0 {
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
        KeyCode::Char('r') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::EditWorkspace;
        }
        KeyCode::Char('e') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::EditItem;
        }
        KeyCode::Char(' ') => {
            let mut index: usize = 0;

            if app.current_workspace != 0 {
                for workspace in app.workspaces[0..app.current_workspace].iter() {
                    index += workspace.num_of_item;
                }
            }

            index += app.current_item;

            let item = app.items.get_mut(index).unwrap();
            if item.is_finished {
                item.is_finished = false;
            } else {
                item.is_finished = true;
            }
        }
        KeyCode::Char('d') => {
            let mut index: usize = 0;

            if app.current_workspace != 0 {
                for workspace in app.workspaces[0..app.current_workspace].iter() {
                    index += workspace.num_of_item;
                }
            }

            index += app.current_item;

            if !app.items.is_empty() {
                app.items.remove(index);
            }

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
        KeyCode::Char('w') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::AddWorkspace;
        }
        KeyCode::Char('-') => {
            app.previous_mode = app.mode;
            app.mode = app::Mode::DisplayWorkspace
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
        (Mode::AddWorkspace, modifiers, keycode) => {
            handle_keys_add_workspace(keycode, modifiers, app)
        }
        (Mode::EditItem, modifiers, keycode) => handle_keys_edit_item(keycode, modifiers, app),
        (Mode::EditWorkspace, modifiers, keycode) => {
            handle_keys_edit_workspace(keycode, modifiers, app)
        }
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
        (Mode::DisplayWorkspace, modifiers, keycode) => {
            handle_keys_display_workspace(keycode, modifiers, app)
        }
    }

    if !app.workspaces.is_empty() {
        app.workspaces
            .get_mut(app.current_workspace)
            .unwrap()
            .num_of_item = app
            .items
            .iter()
            .filter(|i| i.workspace == app.workspaces[app.current_workspace].title)
            .count();
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

    if !app.workspaces.is_empty() {
        for workspace in app.workspaces.iter_mut() {
            if workspace.slot == app.current_workspace {
                workspace.is_selected = true;
            } else {
                workspace.is_selected = false;
            }
        }
    }

    let _ = request_redraw.try_send(());
}
