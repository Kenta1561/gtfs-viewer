use bitflags::bitflags;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};

use crate::db::types::BoardType::{ARRIVAL, DEPARTURE};
use crate::db::types::ExceptionType::{ADDED, NONE, REMOVED};

const TIME_FORMAT: &str = "%H:%M";

pub trait WidgetItem<T> {
    fn to_val(&self) -> T;
}

//region Station
pub struct Station {
    pub stop_id: String,
    pub name: String,
}

impl WidgetItem<String> for Station {
    fn to_val(&self) -> String {
        self.stop_id.to_string()
    }
}
//endregion

//region Stop
pub struct DisplayStop {
    pub arr_time: String,
    pub dep_time: String,
    pub trip_id: u32,
    pub short_name: String,
    pub head_sign: String,
}

impl DisplayStop {
    pub fn from(s: Stop, dt: NaiveDateTime) -> Self {
        Self {
            arr_time: s.get_adjusted_arr(&dt),
            dep_time: s.get_adjusted_dep(&dt),
            trip_id: s.trip_id,
            short_name: s.short_name,
            head_sign: s.head_sign,
        }
    }
}

impl WidgetItem<u32> for DisplayStop {
    fn to_val(&self) -> u32 {
        self.trip_id
    }
}

pub struct Stop {
    pub arrival_time: Duration,
    pub departure_time: Duration,
    pub trip_id: u32,
    //tmp
    pub short_name: String,
    pub service_id: u16,
    pub head_sign: String,
}

impl Stop {
    pub fn is_after_adjusted_time(
        &self, board_type: &BoardType, date_time: &NaiveDateTime
    ) -> bool {
        &self.get_adjusted_dt(board_type, date_time) > date_time
    }

    pub fn get_adjusted_dt(
        &self,
        board_type: &BoardType,
        base_dt: &NaiveDateTime
    ) -> NaiveDateTime {
        let dur_raw = self.get_time_duration(board_type);
        let dur_adjusted = dur_raw - Duration::days(dur_raw.num_days());

        base_dt.date().and_hms(0, 0, 0) + dur_adjusted
    }

    //todo remove later
    pub fn get_adjusted_arr(&self, base_dt: &NaiveDateTime) -> String {
        self.get_adjusted_dt(&ARRIVAL, base_dt).format(TIME_FORMAT).to_string()
    }

    //todo remove later
    pub fn get_adjusted_dep(&self, base_dt: &NaiveDateTime) -> String {
        self.get_adjusted_dt(&DEPARTURE, base_dt).format(TIME_FORMAT).to_string()
    }

    fn get_time_duration(&self, board_type: &BoardType) -> Duration {
        match board_type {
            ARRIVAL => self.arrival_time,
            DEPARTURE => self.departure_time,
        }
    }
}
//endregion

//region Service availability
bitflags! {
    pub struct Weekday: u8 {
        const MON = 0b00000001;
        const TUE = 0b00000010;
        const WED = 0b00000100;
        const THU = 0b00001000;
        const FRI = 0b00010000;
        const SAT = 0b00100000;
        const SUN = 0b01000000;
    }
}

impl Weekday {
    pub fn from_rows(
        mon: bool, tue: bool, wed: bool, thu: bool, fri: bool, sat: bool, sun: bool,
    ) -> Weekday {
        //TODO improvements?
        let mut weekday = Weekday::empty();
        weekday.set(Weekday::MON, mon);
        weekday.set(Weekday::TUE, tue);
        weekday.set(Weekday::WED, wed);
        weekday.set(Weekday::THU, thu);
        weekday.set(Weekday::FRI, fri);
        weekday.set(Weekday::SAT, sat);
        weekday.set(Weekday::SUN, sun);

        weekday
    }

    // 0-monday-based index
    pub fn from_index(index: u32) -> Weekday {
        Weekday::from_bits_truncate(2_i32.pow(index) as u8)
    }
}

enum ExceptionType {
    NONE,
    ADDED,
    REMOVED,
}

pub enum BoardType {
    ARRIVAL,
    DEPARTURE,
}

pub struct Service {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub operating_weekdays: Weekday,
    pub exceptions: Vec<ServiceException>,
}

pub struct ServiceException {
    pub exception_date: NaiveDate,
    pub exception_type: u8,
}

impl Service {
    pub fn new(
        start_date: NaiveDate, end_date: NaiveDate, operating_weekdays: Weekday,
    ) -> Service {
        Service {
            start_date,
            end_date,
            operating_weekdays,
            exceptions: vec![],
        }
    }

    pub fn is_available(&self, date: &NaiveDate) -> bool {
        let weekday = Weekday::from_index(date.weekday().num_days_from_monday());

        match self.get_exception_type_for_date(date) {
            ADDED => true,
            NONE => !(self.operating_weekdays & weekday).is_empty(),
            _ => false
        }
    }

    fn get_exception_type_for_date(&self, date: &NaiveDate) -> ExceptionType {
        let exception = self.exceptions.iter()
            .filter(|e| e.exception_date.eq(date))
            .next();

        if let Some(x) = exception {
            match x.exception_type {
                1 => ADDED,
                2 => REMOVED,
                _ => panic!("Unexpected exception type {}!", x.exception_type),
            }
        } else {
            NONE
        }
    }

}
//endregion
