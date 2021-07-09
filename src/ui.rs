use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};

use crate::ui::Selection::*;

#[derive(PartialEq)]
pub enum Selection {
    SEARCH,
    STATION,
    DATE,
    TIME,
    BOARD,
    TRIP,
}

impl Selection {
    pub fn next(&self) -> Selection {
        match self {
            SEARCH => STATION,
            STATION => DATE,
            DATE => TIME,
            TIME => BOARD,
            BOARD => TRIP,
            TRIP => SEARCH,
        }
    }

    pub fn previous(&self) -> Selection {
        match self {
            SEARCH => TRIP,
            STATION => SEARCH,
            DATE => STATION,
            TIME => DATE,
            BOARD => TIME,
            TRIP => BOARD,
        }
    }

    pub fn right(&self) -> Selection {
        match self {
            BOARD => TRIP,
            TRIP => SEARCH,
            _ => BOARD,
        }
    }

    pub fn left(&self) -> Selection {
        match self {
            TRIP => BOARD,
            BOARD => SEARCH,
            _ => TRIP,
        }
    }
}

pub struct App {
    pub current_block: Selection,
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

fn get_search_field<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .title("Search")
        .border_style(get_border_style(app, Selection::SEARCH))
}

fn get_station_list<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .title("Stations")
        .border_style(get_border_style(app, Selection::STATION))
}

fn get_date_field<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .border_style(get_border_style(app, Selection::DATE))
}

fn get_time_field<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .border_style(get_border_style(app, Selection::TIME))
}
//endregion

//region Center area
pub fn build_center_block<'a>(app: &App) -> Block<'a> {
    //TODO Make toggelable between dep/arr
    get_generic_block()
        .title("Departures")
        .border_style(get_border_style(app, Selection::BOARD))
}
//endregion

//region Right area
pub fn build_right_block<'a>(app: &App) -> Block<'a> {
    get_generic_block()
        .title("Trip")
        .border_style(get_border_style(app, Selection::TRIP))
}
//endregion

//region Utility functions
fn get_border_style(app: &App, block: Selection) -> Style {
    if app.current_block == block {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    }
}

fn get_generic_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
}
//endregion
