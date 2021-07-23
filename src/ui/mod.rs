use std::error::Error;

use chrono::{DateTime, Local};
use rusqlite::ffi::ErrorCode::ConstraintViolation;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, ListState, TableState, Widget};

use crate::db::types::{Station, Stop, WidgetItem, BoardType, DisplayStop};
use crate::ui::board::Board;
use crate::ui::menu::{DateSelection, Search, StationList, TimeSelection};
use crate::ui::SelectableBlock::*;
use crate::handler::KeyHandler;
use crate::ui::trip::Trip;
use crossterm::event::{KeyEvent, KeyCode};
use crate::db::GTFSDatabase;

pub mod menu;
pub mod board;
pub mod trip;

pub trait UIBlock<T>
    where T: Widget
{
    fn build(&self, hovered: bool, selected: bool) -> Result<T, Box<dyn Error>>;
}

//region WidgetData
pub struct WidgetData<T, K, S>
    where T: WidgetItem<K>, S: WidgetState
{
    pub state: S,
    pub items: Vec<T>,

    pub changed: bool,
    pub key: K,
}

impl<T, K, S> WidgetData<T, K, S>
    where T: WidgetItem<K>, S: WidgetState
{
    fn new(key: K) -> Self {
        Self {
            state: S::default(),
            items: Vec::new(),
            changed: true,
            key,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state = S::default();
    }

    //todo handle empty list case
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

    //todo handle empty list case
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

    pub fn get_selected_item(&self) -> Option<&T> {
        self.state.selected().map_or(None, |i| self.items.get(i))
    }

    pub fn update(&mut self) {
        if let Some(item) = self.get_selected_item() {
            self.key = item.to_val();
            self.changed = true;
        }
    }
}
//endregion

//region SelectableBlock
#[derive(Copy, Clone, PartialEq)]
pub enum SelectableBlock {
    SEARCH,
    STATION,
    DATE,
    TIME,
    BOARD,
    TRIP,
}

impl SelectableBlock {
    pub fn next(&self) -> SelectableBlock {
        match self {
            SEARCH => STATION,
            STATION => DATE,
            DATE => TIME,
            TIME => BOARD,
            BOARD => TRIP,
            TRIP => SEARCH,
        }
    }

    pub fn prev(&self) -> SelectableBlock {
        match self {
            SEARCH => TRIP,
            STATION => SEARCH,
            DATE => STATION,
            TIME => DATE,
            BOARD => TIME,
            TRIP => BOARD,
        }
    }

    pub fn right(&self) -> SelectableBlock {
        match self {
            BOARD => TRIP,
            TRIP => SEARCH,
            _ => BOARD,
        }
    }

    pub fn left(&self) -> SelectableBlock {
        match self {
            TRIP => BOARD,
            BOARD => SEARCH,
            _ => TRIP,
        }
    }
}
//endregion

//region WidgetState
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
//endregion

//region App
pub struct App {
    db: GTFSDatabase,
    //Block
    pub block_hover: SelectableBlock,
    pub block_focused: Option<SelectableBlock>,

    pub search: Search,
    pub date_selection: DateSelection,
    pub time_selection: TimeSelection,
    pub station_list: StationList,

    pub board: Board,

    pub trip: Trip,
}

impl App {
    pub fn new(db: GTFSDatabase) -> App {
        App {
            db,
            block_hover: SEARCH,
            block_focused: None,
            search: Search::default(),
            date_selection: DateSelection::default(),
            time_selection: TimeSelection::default(),
            station_list: StationList::default(),
            board: Board::default(),
            trip: Trip::default(),
        }
    }

    pub fn key_handler(&mut self) -> Box<&mut dyn KeyHandler> {
        if let Some(b) = self.block_focused {
            Box::new(match b {
                SelectableBlock::SEARCH => &mut self.search,
                SelectableBlock::STATION => &mut self.station_list,
                SelectableBlock::DATE => &mut self.date_selection,
                SelectableBlock::TIME => &mut self.time_selection,
                SelectableBlock::BOARD => &mut self.board,
                SelectableBlock::TRIP => &mut self.trip,
            })
        } else {
            Box::new(self)
        }
    }

    pub fn render<B>(
        &mut self, frame: &mut Frame<B>, layout: &[Rect],
    ) -> Result<(), Box<dyn Error>>
        where B: Backend
    {
        //StationList
        if self.search.changed {
            self.station_list.data.set_items(
                self.db.fetch_stations(&self.search.input)?
            );
            self.search.changed = false;
        }

        //Board
        if self.station_list.data.changed {
            let selected_dt = self.date_selection.date.and_time(self.time_selection.time);
            let stops = self.db.fetch_stops(
                &self.station_list.data.key,
                BoardType::DEPARTURE,
                selected_dt
            )?.iter().map(|s| DisplayStop::from(s, selected_dt)).collect();
            self.board.data.set_items(stops);
            self.station_list.data.changed = false;
        }

        //Left: Menu
        let menu_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Max(100),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(layout[0]);

        frame.render_widget(
            self.search.build(
                self.block_hover == SelectableBlock::SEARCH,
                self.block_focused == Some(SelectableBlock::SEARCH),
            )?,
            menu_layout[0],
        );

        frame.render_widget(
            self.date_selection.build(
                self.block_hover == SelectableBlock::DATE,
                self.block_focused == Some(SelectableBlock::DATE),
            )?,
            menu_layout[2],
        );

        frame.render_widget(
            self.time_selection.build(
                self.block_hover == SelectableBlock::TIME,
                self.block_focused == Some(SelectableBlock::TIME),
            )?,
            menu_layout[3],
        );

        frame.render_stateful_widget(
            self.station_list.build(
                self.block_hover == SelectableBlock::STATION,
                self.block_focused == Some(SelectableBlock::STATION),
            )?,
            menu_layout[1],
            &mut self.station_list.data.state,
        );

        //Center: Board
        frame.render_stateful_widget(
            self.board.build(
                self.block_hover == SelectableBlock::BOARD,
                self.block_focused == Some(SelectableBlock::BOARD),
            )?,
            layout[1],
            &mut self.board.data.state,
        );

        Ok(())
    }
}

impl KeyHandler for App {
    fn handle_key(&mut self, event: &KeyEvent) {
        match event.code {
            //Direction
            KeyCode::Down | KeyCode::Char('j') => {
                self.block_hover = self.block_hover.next()
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.block_hover = self.block_hover.prev();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.block_hover = self.block_hover.left();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.block_hover = self.block_hover.right();
            }
            //Selection
            KeyCode::Enter => {
                self.block_focused = Some(self.block_hover);
            }
            _ => {}
        }
    }
}
//endregion

//todo improve matching, replace with bitflag
fn create_block<'a>(hovered: bool, selected: bool) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(
                    if hovered {
                        if selected {
                            Color::Magenta
                        } else {
                            Color::Cyan
                        }
                    } else {
                        Color::White
                    }
                )
        )
}
