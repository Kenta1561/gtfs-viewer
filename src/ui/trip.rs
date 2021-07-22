use std::error::Error;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Row, Table, TableState};

use crate::db::GTFSDatabase;
use crate::db::types::{Stop, DisplayStop};
use crate::ui::{App, SelectableBlock, UIBlock, WidgetData, create_block};
use crate::handler::KeyHandler;
use crossterm::event::KeyEvent;

pub struct Trip {
    data: WidgetData<DisplayStop, u32, TableState>,
}

impl Default for Trip {
    fn default() -> Self {
        Self {
            data: WidgetData::new(0),
        }
    }
}

impl KeyHandler for Trip {
    fn handle_key(&mut self, event: &KeyEvent) {
        unimplemented!()
    }
}

impl<'a> UIBlock<Table<'a>> for Trip {
    fn build(&self, hovered: bool, selected: bool) -> Result<Table<'a>, Box<dyn Error>> {
        let rows: Vec<Row> = self.data.items.iter()
            .map(|s| Row::new(vec![
                s.head_sign.to_string(),
                s.arr_time.to_string(),
                s.dep_time.to_string(),
            ]))
            .collect();

        let table = Table::new(rows)
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["Station", "Arr.", "Dep."])
                    .style(Style::default().add_modifier(Modifier::BOLD))
            )
            .highlight_style(Style::default().fg(Color::Magenta))
            .highlight_symbol(">>")
            .block(create_block(hovered, selected))
            .widths(&[
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]);

        Ok(table)
    }
}
