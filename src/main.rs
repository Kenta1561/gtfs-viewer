use std::error::Error;
use std::io::stdout;

use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use crate::handler::handle_key_event;
use crate::ui::App;
use rusqlite::Connection;
use crate::ui::menu::build_menu;
use crate::ui::board::build_board;
use crate::ui::trip::build_trip;

mod handler;
mod ui;
mod db;

//TODO replace later with config field
const DB_PATH: &str = "scripts/data.db";

fn main() -> Result<(), Box<dyn Error>> {
    //DB
    let conn = Connection::open(DB_PATH)?;

    //UI
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ])
                .split(f.size());

            build_menu(&mut app, f, &conn,root_layout[0]).unwrap();
            build_board(&mut app, f, root_layout[1]);
            build_trip(&mut app, f, root_layout[2]);
        })?;

        match read()? {
            Event::Key(e) => match e.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                _ => handle_key_event(&mut app, &e),
            },
            _ => {},
        }
    }
    Ok(())
}
