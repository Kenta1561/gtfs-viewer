use std::collections::HashMap;
use std::error::Error;

use chrono::{Local, NaiveDateTime, Duration};
use rusqlite::Connection;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Row, Table};

use crate::db::GTFSDatabase;
use crate::db::types::{BoardType, Stop};
use crate::ui::{App, get_generic_block, UIBlock};

pub fn build_board<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, area: Rect,
) -> Result<(), Box<dyn Error>>
    where B: Backend
{
    /*app.board_table.items = db.fetch_stops(
        "de:06412:10",
        BoardType::DEPARTURE,
        Local::now().naive_local(),
    )?;*/


    let s1 = Stop {
        arrival_time: Duration::hours(15) + Duration::minutes(10),
        departure_time: Duration::hours(15) + Duration::minutes(12),
        trip_id: 0,
        short_name: "123".to_string(),
        service_id: 0
    };

    app.board_table.items = vec![s1];

    let rows: Vec<Row> = app.board_table.items.iter()
        .map(|s| Row::new(vec![
            s.short_name.to_string(),
            String::from("B"),
            String::from("C"),
        ]))
        .collect();

    let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Col A", "Col B", "Col C"])
        )
        .highlight_style(Style::default().fg(Color::Magenta))
        .highlight_symbol(">>")
        .block(get_generic_block(app, UIBlock::BOARD, Some("Departures")))
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ]);

    frame.render_stateful_widget(table, area, &mut app.board_table.state);

    Ok(())
}
