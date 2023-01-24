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

impl Journey {
    pub fn departure_before_return(&self) -> bool {
        self.departure_date < self.return_date
    }

    pub fn duration_over(&self, limit: chrono::Duration) -> bool {
        self.duration >= limit
    }

    pub fn distance_over(&self, limit: Meters) -> bool {
        self.distance >= limit
    }

    pub fn is_valid(&self) -> bool {
        self.departure_before_return()
            && self.duration_over_or_exactly(chrono::Duration::seconds(10))
            && self.distance_over_or_exactly(Meters(10))
    }
}
