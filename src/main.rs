use std::error::Error;
use std::io::stdout;

use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use crate::handler::handle_key_event;
use crate::ui::{App, build_left_area, build_center_block, build_right_block};

mod handler;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
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

            build_left_area(&app, f, root_layout[0]);
            let block_center = build_center_block(&app);
            let block_right = build_right_block(&app);

            //f.render_widget(block_left, root_layout[0]);
            f.render_widget(block_center, root_layout[1]);
            f.render_widget(block_right, root_layout[2]);
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
            _ => {}
        }
    }

    Ok(())
}

