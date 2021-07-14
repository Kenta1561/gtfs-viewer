use std::error::Error;

use chrono::{Duration, Local};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Row, Table};

use crate::db::GTFSDatabase;
use crate::db::types::{Stop, BoardType};
use crate::ui::{App, get_generic_block, UIBlock};

pub fn build_board<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, area: Rect,
) -> Result<(), Box<dyn Error>>
    where B: Backend
{
    if app.board_table.changed {
        app.board_table.widget.items = db.fetch_stops(
            &app.board_table.trigger,
            BoardType::DEPARTURE,
            app.selected_dt.naive_local(),
        )?;
        app.board_table.changed = false;
    }

    let title = app.station_list.get_selected_item().map(|s| s.name.as_str());

    let rows: Vec<Row> = app.board_table.widget.items.iter()
        .map(|s| Row::new(vec![
            s.short_name.to_string(),
            s.headsign.to_string(),
            s.tmp_get_adjusted_arrival(&app.selected_dt.naive_local()),
            s.tmp_get_adjusted_departure(&app.selected_dt.naive_local()),
        ]))
        .collect();

    let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Nr.", "Destination", "Arrival", "Departure"])
        )
        .highlight_style(Style::default().fg(Color::Magenta))
        .highlight_symbol(">>")
        .block(get_generic_block(app, UIBlock::BOARD, title))
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]);

    frame.render_stateful_widget(table, area, &mut app.board_table.widget.state);

    Ok(())
}
