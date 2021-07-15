use chrono::{Duration, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::App;
use crate::ui::{UIBlock, DependentView, ViewState};

pub fn handle_key_event(app: &mut App, event: &KeyEvent) {
    match event.code {
        KeyCode::Esc => {
            app.block_focused = None;
        }
        _ => match app.block_focused {
            Some(b) => handle_block_nav(app, &b, event),
            None => handle_global_nav(app, event),
        },
    }
}

fn handle_global_nav(app: &mut App, event: &KeyEvent) {
    match event.code {
        //Direction
        KeyCode::Down | KeyCode::Char('j') => {
            app.block_hover = app.block_hover.next()
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.block_hover = app.block_hover.prev();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.block_hover = app.block_hover.left();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.block_hover = app.block_hover.right();
        }
        //Selection
        KeyCode::Enter => {
            app.block_focused = Some(app.block_hover);
        }
        _ => {}
    }
}

//TODO improve pattern matching
fn handle_block_nav(app: &mut App, block: &UIBlock, event: &KeyEvent) {
    match block {
        UIBlock::DATE => handle_date(app, event),
        UIBlock::TIME => handle_time(app, event),
        UIBlock::SEARCH => handle_search(app, event),
        UIBlock::STATION => handle_station(app, event),
        UIBlock::BOARD => handle_board(app, event),
        _ => {}
    }
}

fn handle_scroll_nav<I, S, T>(view: &mut DependentView<I, S, T>, code: &KeyCode)
    where S: ViewState
{
    match code {
        KeyCode::Down | KeyCode::Char('j') => view.widget.next(),
        KeyCode::Up | KeyCode::Char('k') => view.widget.prev(),
        KeyCode::Home => view.widget.start(),
        KeyCode::End => view.widget.end(),
        _ => {}
    }
}

//region Menu
fn handle_date(app: &mut App, event: &KeyEvent) {
    app.selected_dt = match event.code {
        KeyCode::Left | KeyCode::Char('h') => app.selected_dt - Duration::days(1),
        KeyCode::Right | KeyCode::Char('l') => app.selected_dt + Duration::days(1),
        KeyCode::Char('t') => Local::today().and_time(app.selected_dt.time()).unwrap(),
        _ => app.selected_dt,
    }
}

fn handle_time(app: &mut App, event: &KeyEvent) {
    app.selected_dt = match event.code {
        KeyCode::Left | KeyCode::Char('h') => {
            app.selected_dt - get_modified_duration(&event.modifiers)
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.selected_dt + get_modified_duration(&event.modifiers)
        }
        KeyCode::Char('n') => Local::now(),
        _ => app.selected_dt,
    }
}

fn get_modified_duration(modifiers: &KeyModifiers) -> Duration {
    Duration::minutes(
        if modifiers.contains(KeyModifiers::SHIFT) { 5 } else { 60 }
    )
}

fn handle_search(app: &mut App, event: &KeyEvent) {
    match event.code {
        KeyCode::Backspace => {
            app.station.trigger.remove(app.station.trigger.len() - 1);
        }
        KeyCode::Char(c) => {
            if c == 'u' && event.modifiers.contains(KeyModifiers::CONTROL) {
                app.station.trigger.clear();
            } else {
                app.station.trigger.push(c);
            }
        }
        KeyCode::Enter => {
            app.station.changed = true;
        }
        _ => {}
    }
}

//TODO temporary, make generic?
fn handle_station(app: &mut App, event: &KeyEvent) {
    match event.code {
        KeyCode::Enter => app.update_board(),
        x => handle_scroll_nav(&mut app.station, &x)
    }
}
//endregion

fn handle_board(app: &mut App, event: &KeyEvent) {
    match event.code {
        x => handle_scroll_nav(&mut app.board, &x)
    }
}
