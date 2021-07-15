use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect, Constraint};

use crate::ui::{App, get_generic_block, UIBlock};
use crate::db::GTFSDatabase;
use tui::widgets::{Row, Table};
use tui::style::{Style, Color};
use std::error::Error;

pub fn build_trip<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, area: Rect
) -> Result<(), Box<dyn Error>>
    where B: Backend
{
    app.trip.widget.items = db.fetch_trip(1434408949)?;

    let rows: Vec<Row> = app.trip.widget.items.iter()
        .map(|s| Row::new(vec![
            s.headsign.to_string(),
            s.tmp_get_adjusted_arrival(&app.selected_dt.naive_local()),
            s.tmp_get_adjusted_departure(&app.selected_dt.naive_local()),
        ]))
        .collect();

    let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Station", "Arr.", "Dep."])
        )
        .highlight_style(Style::default().fg(Color::Magenta))
        .highlight_symbol(">>")
        .block(get_generic_block(app, UIBlock::TRIP, Some("Trip details")))
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

    frame.render_stateful_widget(table, area, &mut app.trip.widget.state);

    Ok(())
}
