use chrono::{Duration, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::App;
use crate::ui::{SelectableBlock, WidgetState, WidgetData};
use crate::db::types::WidgetItem;

pub trait KeyHandler {
    fn handle_key(&mut self, event: &KeyEvent);
}

pub fn scroll_nav<T, K, S>(data: &mut WidgetData<T, K, S>, code: &KeyCode)
    where T: WidgetItem<K>, S: WidgetState
{
    match code {
        KeyCode::Char('j') | KeyCode::Down => {
            data.next();
        },
        KeyCode::Char('k') | KeyCode::Up => {
            data.prev();
        },
        KeyCode::Home => {
            data.start();
        },
        KeyCode::End => {
            data.end();
        },
        KeyCode::Enter => {
            if let Some(x) = data.get_selected_item() {
                data.key = x.to_val();
                data.changed = true;
            }
        },
        _ => {},
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
