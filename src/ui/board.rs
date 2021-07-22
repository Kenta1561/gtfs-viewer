use std::error::Error;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Row, Table, TableState};

use crate::db::GTFSDatabase;
use crate::db::types::{BoardType, Stop, DisplayStop};
use crate::ui::{App, create_block, SelectableBlock, UIBlock, WidgetData};
use crate::handler::{KeyHandler, scroll_nav};
use crossterm::event::KeyEvent;

pub struct Board {
    pub data: WidgetData<DisplayStop, u32, TableState>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            data: WidgetData::new(0),
        }
    }
}

impl KeyHandler for Board {
    fn handle_key(&mut self, event: &KeyEvent) {
        scroll_nav(&mut self.data, &event.code);
    }
}

impl<'a> UIBlock<Table<'a>> for Board {
    fn build(&self, hovered: bool, selected: bool) -> Result<Table<'a>, Box<dyn Error>> {
        let rows: Vec<Row> = self.data.items.iter()
            .map(|s| Row::new(vec![
                s.trip_id.to_string(),
                s.short_name.to_string(),
                s.head_sign.to_string(),
                s.arr_time.to_string(),
                s.dep_time.to_string(),
            ]))
            .collect();

        let table = Table::new(rows)
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["ID", "Nr.", "Destination", "Arr.", "Dep."])
                    .style(Style::default().add_modifier(Modifier::BOLD))
            )
            .highlight_style(Style::default().fg(Color::Magenta))
            .highlight_symbol(">>")
            .block(create_block(hovered, selected))
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]);

        Ok(table)
    }
}
