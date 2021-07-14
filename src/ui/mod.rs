use chrono::{DateTime, Local};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, ListState, Row, TableState};

use crate::db::types::{Station, Stop};
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

pub struct DependentView<I, S, T>
    where S: ViewState
{
    pub trigger: T,
    pub widget: StatefulView<I, S>,
    pub changed: bool,
}

impl<I, S, T> DependentView<I, S, T>
    where S: ViewState
{
    pub fn empty(trigger: T) -> DependentView<I, S, T> {
        DependentView {
            trigger,
            widget: StatefulView::empty(),
            changed: true,  //true for initialization
        }
    }

    pub fn get_selected_item(&self) -> Option<&I> {
        if let Some(i) = self.widget.state.selected() {
            return self.widget.items.get(i)
        }
        None
    }
}

pub trait ViewState: Default {
    fn select(&mut self, index: Option<usize>);
    fn selected(&self) -> Option<usize>;
}

impl ViewState for ListState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }

    fn selected(&self) -> Option<usize> {
        self.selected()
    }
}

impl ViewState for TableState {
    fn select(&mut self, index: Option<usize>) {
        self.select(index);
    }

    fn selected(&self) -> Option<usize> {
        self.selected()
    }
}

pub struct StatefulView<T, S: ViewState> {
    pub items: Vec<T>,
    pub state: S,
}

impl<T, S: ViewState> StatefulView<T, S> {
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

    fn get_selected_item(&self) -> Option<&T> {
        if let Some(i) = self.state.selected() {
            self.items.get(i)
        } else {
            None
        }
    }

    //region Navigation
    pub fn next(&mut self) {
        self.state.select(Some(
            match self.state.selected() {
                Some(i) => {
                    if i < self.items.len() - 1 {
                        i + 1
                    } else {
                        0
                    }
                }
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
                }
                None => 0,
            }
        ));
    }

    pub fn start(&mut self) {
        self.state.select(Some(0));
    }

    pub fn end(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
    //endregion
}

pub struct App {
    //Block
    pub block_hover: UIBlock,
    pub block_focused: Option<UIBlock>,

    //Raw data
    pub selected_dt: DateTime<Local>,

    //Stateful views
    pub stations: DependentView<Station, ListState, String>,
    pub board: DependentView<Stop, TableState, String>,
}

impl App {
    pub fn new() -> App {
        App {
            block_hover: SEARCH,
            block_focused: None,
            selected_dt: Local::now(),
            stations: DependentView::empty(String::new()),
            board: DependentView::empty(String::new()),
        }
    }

    pub fn update_board(&mut self) {
        let selected_item = self.stations.get_selected_item().unwrap();
        self.board.trigger = selected_item.stop_id.to_string();
        self.board.changed = true;
    }
}

//region Utility methods
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
