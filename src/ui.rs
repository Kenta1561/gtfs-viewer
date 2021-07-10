use std::borrow::Borrow;
use std::error::Error;

use chrono::{DateTime, Local};
use rusqlite::Connection;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style, Modifier};
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::db::get_stations;
use crate::db::items::Stop;
use crate::ui::UIBlock::*;

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
    //Block
    pub block_hover: UIBlock,
    pub block_focused: Option<UIBlock>,

    //Raw data
    pub selected_dt: DateTime<Local>,
    pub input_change: bool,
    pub input: String,

    //List states
    pub station_list: StatefulList<Stop>,
}

impl App {
    pub fn new() -> App {
        App {
            block_hover: SEARCH,
            block_focused: None,
            selected_dt: Local::now(),
            input: String::new(),
            input_change: true,
            station_list: StatefulList::empty(),
        }
    }

    //region Input string manipulation
    pub fn input_remove(&mut self) {
        if self.input.len() > 0 {
            self.input.truncate(self.input.len() - 1);
        }
    }

    pub fn input_clear(&mut self) {
        self.input.clear();
        self.notify_input_change();
    }

    pub fn input_add(&mut self, c: char) {
        //TODO temporary solution
        if c.is_ascii_alphabetic() {
            self.input.push(c);
        }
    }

    pub fn notify_input_change(&mut self) {
        self.input_change =  true;
    }
    //endregion
}

pub struct StatefulList<T> {
    pub items: Vec<T>,
    state: ListState,
}

//https://docs.rs/tui/0.15.0/tui/widgets/trait.StatefulWidget.html
impl<T> StatefulList<T> {
    fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            items,
            state: ListState::default(),
        }
    }

    fn empty() -> StatefulList<T> {
        Self::new(Vec::new())
    }

    fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state = ListState::default();
    }

    //TODO solve: Always none!
    pub fn next(&mut self) {
        self.state.select(Some(
            match self.state.selected() {
                Some(i) => {
                    if i < self.items.len() - 1 {
                        i + 1
                    } else {
                        0
                    }
                },
                None => 0,
            }
        ));
    }

    pub fn prev(&mut self) {
        self.state.select(Some(
            match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                },
                None => 0,
            }
        ));
    }
}

//region Left area
pub fn build_left_area<B>(
    app: &mut App, frame: &mut Frame<B>, db: &Connection, root_area: Rect,
) -> Result<(), Box<dyn Error>>
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
    render_station_list(app, frame, layout[1], db);
    frame.render_widget(get_date_field(app), layout[2]);
    frame.render_widget(get_time_field(app), layout[3]);

    Ok(())
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

fn render_station_list<B>(app: &mut App, frame: &mut Frame<B>, area: Rect, db: &Connection)
    where B: Backend
{
    if app.input_change {
        app.station_list.set_items(get_stations(db, &app.input).unwrap());
        app.input_change = false;
    }

    let items: Vec<ListItem> = app.station_list.items.iter()
        .map(|s| ListItem::new(s.name.as_ref()))
        .collect();

    let list = List::new(items)
        .block(get_generic_block(app, UIBlock::STATION, Some("Stations")))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>")
        .highlight_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        );

    frame.render_stateful_widget(list, area, &mut app.station_list.state);
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
        .alignment(Alignment::Center)
}
//endregion

//region Center area
pub fn build_center_block<'a>(app: &App) -> Block<'a> {
    //TODO Make toggelable between dep/arr
    get_generic_block(app, UIBlock::BOARD, Some("Departures"))
}
//endregion

//region Right area
pub fn build_right_block<'a>(app: &App) -> Block<'a> {
    get_generic_block(app, UIBlock::TRIP, Some("Trip"))
}
//endregion

//region Utility functions
fn get_border_style(app: &App, block: UIBlock) -> Style {
    if let Some(b) = app.block_focused {
        if b == block {
            return Style::default().fg(Color::Magenta);
        }
    } else if block == app.block_hover {
        return Style::default().fg(Color::Cyan);
    }

    Style::default().fg(Color::White)
}

fn get_generic_block<'a>(app: &App, block: UIBlock, title: Option<&'a str>) -> Block<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(get_border_style(app, block));

    match title {
        Some(t) => block.title(t),
        None => block
    }
}
//endregion
