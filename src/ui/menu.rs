use std::borrow::Borrow;
use std::error::Error;

use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Duration};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::db::GTFSDatabase;
use crate::db::types::Station;
use crate::ui::{App, create_block, SelectableBlock, UIBlock, WidgetData};
use crate::handler::{KeyHandler, scroll_nav};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

//region StationList
pub struct StationList {
    pub data: WidgetData<Station, String, ListState>,
}

impl Default for StationList {
    fn default() -> Self {
        Self {
            data: WidgetData::new(String::new()),
        }
    }
}

impl KeyHandler for StationList {
    fn handle_key(&mut self, event: &KeyEvent) {
        scroll_nav(&mut self.data, &event.code);
    }
}

impl<'a> UIBlock<List<'a>> for StationList {
    fn build(&self, hovered: bool, selected: bool) -> Result<List<'a>, Box<dyn Error>> {
        let items: Vec<ListItem> = self.data.items.iter()
            .map(|s| ListItem::new(s.name.to_string()))
            .collect();

        Ok(List::new(items)
            .block(create_block(hovered, selected))
            .style(Style::default().fg(Color::White))
            .highlight_symbol(">>")
            .highlight_style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)
            )
        )
    }
}
//endregion

//region DateSelection
pub struct DateSelection {
    pub date: NaiveDate,
}

impl Default for DateSelection {
    fn default() -> Self {
        Self {
            date: Local::now().naive_local().date(),
        }
    }
}

impl KeyHandler for DateSelection {
    fn handle_key(&mut self, event: &KeyEvent) {
        self.date = match event.code {
            KeyCode::Left | KeyCode::Char('h') => self.date - Duration::days(1),
            KeyCode::Right | KeyCode::Char('l') => self.date + Duration::days(1),
            KeyCode::Char('t') => Local::today().naive_local(),
            _ => self.date
        }
    }
}

impl<'a> UIBlock<Paragraph<'a>> for DateSelection {
    fn build(&self, hovered: bool, selected: bool) -> Result<Paragraph<'a>, Box<dyn Error>> {
        let text = Text::from(self.date.format("%Y-%m-%d").to_string());
        Ok(Paragraph::new(text)
            .block(create_block(hovered, selected))
            .alignment(Alignment::Center)
        )
    }
}
//endregion

//region TimeSelection
pub struct TimeSelection {
    pub time: NaiveTime,
}

impl Default for TimeSelection {
    fn default() -> Self {
        Self {
            time: Local::now().naive_local().time(),
        }
    }
}

impl KeyHandler for TimeSelection {
    fn handle_key(&mut self, event: &KeyEvent) {
        self.time = match event.code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.time - get_modified_duration(&event.modifiers)
            },
            KeyCode::Right | KeyCode::Char('l') => {
                self.time + get_modified_duration(&event.modifiers)
            },
            KeyCode::Char('n') => Local::now().time(),
            _ => self.time,
        }
    }
}

fn get_modified_duration(modifiers: &KeyModifiers) -> Duration {
    Duration::minutes(
        if modifiers.contains(KeyModifiers::SHIFT) { 5 } else { 60 }
    )
}

impl<'a> UIBlock<Paragraph<'a>> for TimeSelection {
    fn build(&self, hovered: bool, selected: bool) -> Result<Paragraph<'a>, Box<dyn Error>> {
        let text = Text::from(self.time.format("%H:%M").to_string());
        Ok(Paragraph::new(text)
            .block(create_block(hovered, selected))
            .alignment(Alignment::Center)
        )
    }
}
//endregion

//region Search
pub struct Search {
    pub input: String,
    //todo deviates from standard location for changed bool
    pub changed: bool,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            input: String::new(),
            changed: true,
        }
    }
}

impl KeyHandler for Search {
    fn handle_key(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Backspace => {
                self.input.remove(self.input.len() - 1);
            }
            KeyCode::Char(c) => {
                if c == 'u' && event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.input.clear();
                } else {
                    self.input.push(c);
                }
            }
            KeyCode::Enter => {
                self.changed = true;
            }
            _ => {}
        }
    }
}

impl<'a> UIBlock<Paragraph<'a>> for Search {
    fn build(&self, hovered: bool, selected: bool) -> Result<Paragraph<'a>, Box<dyn Error>> {
        let text = Text::from(self.input.to_string());
        Ok(Paragraph::new(text)
            .block(create_block(hovered, selected))
            .alignment(Alignment::Left)
        )
    }
}
//endregion
