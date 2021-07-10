use rusqlite::{Connection, Result};
use crate::db::items::Stop;
use crate::db::queries::get_station_query;

mod queries;
pub mod items;

pub fn get_stations(db: &Connection, input: &str) -> Result<Vec<Stop>> {
    //todo tmp block
    let constructed_query = get_station_query(input);
    //println!("{}", constructed_query);
    let mut stmt = db.prepare(&constructed_query)?;
    let iter = stmt.query_map([], |row| {
        Ok(Stop {
            stop_id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(iter.map(|s| s.unwrap()).collect())
}
