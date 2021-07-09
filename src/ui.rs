use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect, Alignment};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};

use crate::ui::UIBlock::*;
use chrono::{Local, DateTime};
use tui::text::Text;
use std::borrow::Borrow;

#[derive(Copy, Clone, PartialEq)]
pub enum UIBlock {
    SEARCH,
    STATION,
    DATE,
    TIME,
    BOARD,
    TRIP,
}

impl UIBlock {
    pub fn next(&self) -> UIBlock {
        match self {
            SEARCH => STATION,
            STATION => DATE,
            DATE => TIME,
            TIME => BOARD,
            BOARD => TRIP,
            TRIP => SEARCH,
        }
    }

    pub fn prev(&self) -> UIBlock {
        match self {
            SEARCH => TRIP,
            STATION => SEARCH,
            DATE => STATION,
            TIME => DATE,
            BOARD => TIME,
            TRIP => BOARD,
        }
    }

    pub fn right(&self) -> UIBlock {
        match self {
            BOARD => TRIP,
            TRIP => SEARCH,
            _ => BOARD,
        }
    }

    pub fn left(&self) -> UIBlock {
        match self {
            TRIP => BOARD,
            BOARD => SEARCH,
            _ => TRIP,
        }
    }
}

//TODO Move App to other module?
pub struct App {
    pub block_hover: UIBlock,
    pub block_focused: Option<UIBlock>,
    pub selected_dt: DateTime<Local>,
    pub input: String,
}

impl App {
    pub fn new() -> App {
        App {
            block_hover: SEARCH,
            block_focused: None,
            selected_dt: Local::now(),
            input: String::new(),
        }
    }
}

//region Left area
pub fn build_left_area<B>(app: &App, frame: &mut Frame<B>, root_area: Rect)
    where B: Backend
{
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Max(100),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(root_area);

    frame.render_widget(get_search_field(app), layout[0]);
    frame.render_widget(get_station_list(app), layout[1]);
    frame.render_widget(get_date_field(app), layout[2]);
    frame.render_widget(get_time_field(app), layout[3]);
}

fn get_search_field(app: &App) -> Paragraph {
    let text = Text::from(app.input.borrow());  //todo right func?
    Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(get_border_style(app, UIBlock::SEARCH))
            .title("Search")
        )
        .alignment(Alignment::Left)
}

fn get_station_list<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .title("Stations")
        .border_style(get_border_style(app, UIBlock::STATION))
}

fn get_date_field(app: &App) -> Paragraph {
    let text = Text::from(app.selected_dt.format("%Y-%m-%d").to_string());
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(get_border_style(app, UIBlock::DATE)))
        .alignment(Alignment::Center)
}

fn get_time_field(app: &App) -> Paragraph {
    let text = Text::from(app.selected_dt.format("%H:%M").to_string());
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(get_border_style(app, UIBlock::TIME)))
        .alignment(Alignment::Left)
}
//endregion

//region Center area
pub fn build_center_block<'a>(app: &App) -> Block<'a> {
    //TODO Make toggelable between dep/arr
    get_generic_block()
        .title("Departures")
        .border_style(get_border_style(app, UIBlock::BOARD))
}
//endregion

//region Right area
pub fn build_right_block<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .title("Trip")
        .border_style(get_border_style(app, UIBlock::TRIP))
}
//endregion

//region Utility functions
fn get_border_style(app: &App, block: UIBlock) -> Style {
    if let Some(b) = app.block_focused {
        if b == block {
            return Style::default().fg(Color::Magenta)
        }
    } else if block ==  app.block_hover {
        return Style::default().fg(Color::Cyan)
    }

    Style::default().fg(Color::White)
}

fn get_generic_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
}
//endregion
