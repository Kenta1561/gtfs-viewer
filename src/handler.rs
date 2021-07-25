use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{WidgetState, WidgetData};
use crate::db::types::WidgetItem;

pub trait KeyHandler {
    fn handle_key(&mut self, event: &KeyEvent);
}

pub fn scroll_nav<T, K, S>(data: &mut WidgetData<T, K, S>, code: &KeyCode)
    where T: WidgetItem<K>, S: WidgetState
{
    match code {
        KeyCode::Down | KeyCode::Char('j') => {
            data.next();
        },
        KeyCode::Up | KeyCode::Char('k') => {
            data.prev();
        },
        KeyCode::Home => {
            data.start();
        },
        KeyCode::End => {
            data.end();
        },
        KeyCode::Enter => {
            data.update();
        },
        _ => {},
    }
}
