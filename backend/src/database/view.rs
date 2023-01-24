use crate::model::{city, station, station_operator};
use crate::unit::{Coordinate, Kilometers, Minutes};

#[cfg(feature = "postgres")]
pub use crate::database::postgres_view::*;

#[derive(Debug)]
pub struct JourneyListItem {
    pub departure_station: JourneyListItemStation,
    pub return_station: JourneyListItemStation,

    pub distance_kilometers: Kilometers,
    pub duration_minutes: Minutes,
}

#[derive(Debug)]
pub struct JourneyListItemStation {
    pub station_id: station::Id,
    pub name: station::Name,
}

#[derive(Debug)]
pub struct StationListItem {
    pub station_id: station::Id,

    pub city_name: city::Name,
    pub operator_name: station_operator::Name,

    pub name: station::Name,
    pub capacity: station::Capacity,
}

#[derive(Debug)]
pub struct Station {
    pub station_id: station::Id,

    pub name: station::Name,
    pub address: station::Address,

    pub city: city::Name,

    pub total_starting_journeys: u64,
    pub total_ending_journeys: u64,
}

#[derive(Debug)]
pub struct StationListParams {
    pub order_by: StationListOrder,
    pub order_direction: OrderDirection,
}
