use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use chrono::{Duration, NaiveDateTime};
use regex::Regex;
use rusqlite::{Connection, Result};

use crate::db::queries::{get_station_query, SERVICE_QUERY, STOP_QUERY};
use crate::db::types::{BoardType, Service, ServiceException, Station, Stop, Weekday};
use crate::db::util::{str_to_date, str_to_dur};

mod queries;
mod util;
pub mod types;

pub struct GTFSDatabase {
    db: Connection,
    services: HashMap<u16, Service>,
    time_regex: Regex,
}

impl GTFSDatabase {

    pub fn new(db_path: &str) -> Result<GTFSDatabase, Box<dyn Error>> {
        let db = Connection::open(db_path)?;
        //todo service fetching disabled for fast debug
        //let services = fetch_services(&db)?;
        Ok(GTFSDatabase {
            db,
            services: HashMap::new(),
            time_regex: Regex::new(r"(?P<hours>\d{1,2}):(?P<minutes>\d{2}):(?P<seconds>\d{2})")?,
        })
    }

    pub fn fetch_stations(&self, input: &str) -> Result<Vec<Station>> {
        let mut stmt = self.db.prepare(&get_station_query(input))?;
        let iter = stmt.query_map([], |row| {
            Ok(Station {
                stop_id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(iter.map(|s| s.unwrap()).collect())
    }

    pub fn fetch_stops(
        &self, stop_id: &str, board_type: BoardType, date_time: NaiveDateTime,
    ) -> Result<Vec<Stop>, Box<dyn Error>> {
        let mut stmt = self.db.prepare(STOP_QUERY)?;
        let iter = stmt.query_map([stop_id], |row| {
            Ok(Stop {
                arrival_time: str_to_dur(&self.time_regex, row.get(0)?).unwrap(),
                departure_time: str_to_dur(&self.time_regex, row.get(1)?).unwrap(),
                trip_id: row.get(2)?,
                short_name: row.get(4)?,
                service_id: row.get(3)?,
            })
        })?;

        let mut stops: Vec<Stop> = iter.map(|s| s.unwrap())
            // F0: Remove unavailable service
            .filter(|s| self.services.get(&s.service_id).unwrap().is_available(
                &(date_time.date() - Duration::days(s.arrival_time.num_days()))
            ))
            // F1: Apply time filter
            .filter(|s| s.is_after_adjusted_time(&board_type, &date_time))
            //todo: line below for testing, remove later
            .take(100)
            .collect();

        stops.sort_by(|a, b| a.get_adjusted_dt(&board_type, &date_time).cmp(
            &b.get_adjusted_dt(&board_type, &date_time)
        ));

        Ok(stops)
    }
}

pub fn fetch_services(db: &Connection) -> Result<HashMap<u16, Service>, Box<dyn Error>> {
    let mut stmt = db.prepare(SERVICE_QUERY)?;

    let query_time = Instant::now();
    let mut rows = stmt.query([])?;
    println!("Query: {} ms", query_time.elapsed().as_millis());

    let mut map: HashMap<u16, Service> = HashMap::new();

    while let Some(row) = rows.next()? {
        let service_id = row.get(0)?;

        // Components
        let exception = match row.get::<usize, String>(10) {
            Ok(x) => Some(
                ServiceException {
                    exception_date: str_to_date(x)?,
                    exception_type: row.get(11)?,
                }
            ),
            Err(_) => None,
        };

        if !map.contains_key(&service_id) {
            let operating_weekdays = Weekday::from_rows(
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            );

            let service = Service::new(
                str_to_date(row.get(8)?)?,
                str_to_date(row.get(9)?)?,
                operating_weekdays,
            );

            map.insert(service_id, service);
        }

        // Add exception to service exceptions if exists
        if let Some(x) = exception {
            map.get_mut(&service_id).unwrap().exceptions.push(x);
        }
    }

    Ok(map)
}
