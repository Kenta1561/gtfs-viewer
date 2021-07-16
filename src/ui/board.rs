use std::error::Error;

use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Row, Table};

use crate::db::GTFSDatabase;
use crate::db::types::BoardType;
use crate::ui::{App, get_generic_block, UIBlock};

pub fn build_board<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, area: Rect,
) -> Result<(), Box<dyn Error>>
    where B: Backend
{
    if app.board.changed {
        app.board.widget.items = db.fetch_stops(
            &app.board.trigger,
            BoardType::DEPARTURE,
            app.selected_dt.naive_local(),
        )?;
        app.board.changed = false;
    }

    let title = app.station.get_selected_item().map(|s| s.name.as_str());

    let rows: Vec<Row> = app.board.widget.items.iter()
        .map(|s| Row::new(vec![
            s.trip_id.to_string(),
            s.short_name.to_string(),
            s.headsign.to_string(),
            s.get_adjusted_arr(&app.selected_dt.naive_local()),
            s.get_adjusted_dep(&app.selected_dt.naive_local()),
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
        .block(get_generic_block(app, UIBlock::BOARD, title))
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]);

    frame.render_stateful_widget(table, area, &mut app.board.widget.state);

    Ok(())
}
