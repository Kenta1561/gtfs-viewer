use std::error::Error;
use std::io::stdout;

use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use crate::db::GTFSDatabase;
use crate::handler::KeyHandler;
use crate::ui::App;

mod handler;
mod ui;
mod db;

//TODO replace later with config field
const DB_PATH: &str = "scripts/ice.db";

fn main() -> Result<(), Box<dyn Error>> {
    //DB
    let db = GTFSDatabase::new(DB_PATH)?;

    //UI
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(db);

    loop {
        terminal.draw(|f| {
            let root_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    // Constraint::Percentage(30),
                ])
                .split(f.size());

            app.render(f, root_layout.as_slice()).unwrap();
        })?;

        match read()? {
            Event::Key(e) => match e.code {
                //These events should override block-specific ones
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                },
                KeyCode::Esc => {
                    app.block_focused = None;
                },
                _ => app.key_handler().handle_key(&e)
            },
            _ => {},
        }
    }
    Ok(())
}
