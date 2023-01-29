use crate::model::station;
use crate::unit::{Meters, Seconds};

pub type Id = i64;

#[derive(Debug)]
pub struct Journey {
    pub id: Id,

    pub departure_date: chrono::NaiveDateTime,
    pub departure_station_id: station::Id,

    pub return_date: chrono::NaiveDateTime,
    pub return_station_id: station::Id,

    pub distance: Meters,
    pub duration: Seconds,
}

#[derive(Debug)]
pub struct JourneyInsert {
    pub departure_date: chrono::NaiveDateTime,
    pub departure_station_id: station::Id,

    pub return_date: chrono::NaiveDateTime,
    pub return_station_id: station::Id,

    pub distance: Meters,
    pub duration: Seconds,
}

impl JourneyInsert {
    // Note: The following constraints could also be added to the database.
    pub fn departure_before_return(&self) -> bool {
        self.departure_date < self.return_date
    }

    pub fn duration_over_or_exactly(&self, limit: Seconds) -> bool {
        self.duration >= limit
    }

    pub fn distance_over_or_exactly(&self, limit: Meters) -> bool {
        self.distance >= limit
    }

    pub fn is_valid(&self) -> bool {
        self.departure_before_return()
            && self.duration_over_or_exactly(10.0 as Seconds)
            && self.distance_over_or_exactly(10.0 as Meters)
    }
}
