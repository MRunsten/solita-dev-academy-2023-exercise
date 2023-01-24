use crate::model::station;
use crate::unit::Meters;

pub type Id = i64;

#[derive(Debug)]
pub struct Journey {
    pub id: Id,

    pub departure_date: chrono::NaiveDateTime,
    pub departure_station_id: station::Id,

    pub return_date: chrono::NaiveDateTime,
    pub return_station_id: station::Id,

    pub distance: Meters,
    pub duration: chrono::Duration,
}
