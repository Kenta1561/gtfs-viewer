use std::error::Error;

use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Row, Table, TableState};

use crate::db::types::DisplayStop;
use crate::ui::{UIBlock, WidgetData, create_block};
use crate::handler::{KeyHandler, scroll_nav};
use crossterm::event::KeyEvent;

pub struct Trip {
    pub data: WidgetData<DisplayStop, u32, TableState>,
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
        scroll_nav(&mut self.data, &event.code);
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
