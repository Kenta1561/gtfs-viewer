use chrono::{DateTime, Local};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, ListState, TableState};

use crate::db::types::Station;
use crate::ui::UIBlock::*;

pub mod menu;
pub mod board;
pub mod trip;

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

pub struct App {
    //Block
    pub block_hover: UIBlock,
    pub block_focused: Option<UIBlock>,

    //Raw data
    pub selected_dt: DateTime<Local>,
    pub input_change: bool,
    pub input: String,

    //List states
    pub station_list: StatefulView<Station, ListState>,
}

impl App {
    pub fn new() -> App {
        App {
            block_hover: SEARCH,
            block_focused: None,
            selected_dt: Local::now(),
            input: String::new(),
            input_change: true,
            station_list: StatefulView::empty(),
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

pub trait WidgetState: Default {
    fn select(&mut self, index: Option<usize>);
    fn selected(&self) -> Option<usize>;
}

impl WidgetState for ListState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }

    fn selected(&self) -> Option<usize> {
        self.selected()
    }
}

impl WidgetState for TableState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }

    fn selected(&self) -> Option<usize> {
        self.selected()
    }
}

pub struct StatefulView<T, S: WidgetState> {
    pub items: Vec<T>,
    state: S,
}

impl<T, S: WidgetState> StatefulView<T, S> {
    fn new(items: Vec<T>) -> StatefulView<T, S> {
        StatefulView {
            items,
            state: S::default(),
        }
    }

    fn empty() -> StatefulView<T, S> {
        Self::new(Vec::new())
    }

    fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state = S::default();
    }

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
