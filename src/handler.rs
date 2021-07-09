use crossterm::event::KeyCode;
use crate::App;

pub fn handle_key(app: &mut App, code: &KeyCode) {
    match code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.current_block = app.current_block.next();
        },
        KeyCode::Up | KeyCode::Char('k') => {
            app.current_block = app.current_block.previous();
        },
        KeyCode::Left | KeyCode::Char('h') => {
            app.current_block = app.current_block.left();
        },
        KeyCode::Right | KeyCode::Char('l') => {
            app.current_block = app.current_block.right();
        }
        _ => {},
    }
}
