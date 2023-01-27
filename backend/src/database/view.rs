use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::model::{city, station, station_operator};
use crate::unit::{Kilometers, Minutes};

#[cfg(feature = "postgres")]
pub use crate::database::postgres_view::*;

#[derive(Debug, Serialize)]
pub struct JourneyListItem {
    pub departure_date: chrono::DateTime<Utc>,
    pub return_date: chrono::DateTime<Utc>,

    pub departure_station: JourneyListItemStation,
    pub return_station: JourneyListItemStation,

    pub distance_kilometers: Kilometers,
    pub duration_minutes: Minutes,
}

#[derive(Debug, Serialize)]
pub struct JourneyListItemStation {
    pub station_id: station::Id,
    pub name: station::Name,
}

#[derive(Debug, Serialize)]
pub struct StationListItem {
    pub station_id: station::Id,

    pub city_name: city::Name,
    pub operator_name: station_operator::Name,

    pub name: station::Name,
    pub capacity: station::Capacity,
}

#[derive(Debug, Serialize)]
pub struct Station {
    pub station_id: station::Id,

    pub name: station::Name,
    pub address: station::Address,

    pub city: city::Name,

    pub total_starting_journeys: u64,
    pub total_ending_journeys: u64,
}

#[derive(Debug, Deserialize)]
pub struct StationListParams {

    // Note: Depending on how much dynamicity is required the
    // `order_*` parameters may have to be refactored to a single data structure
    // as for example the postgres ORDER BY command supports multiple columns, each with differing
    // directionality.
    pub order_by: StationListOrder,
    pub order_direction: OrderDirection,

    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct JourneyListParams {

    // Note: Depending on how much dynamicity is required the
    // `order_*` parameters may have to be refactored to a single data structure
    // as for example the postgres ORDER BY command supports multiple columns, each with differing
    // directionality.
    pub order_by: JourneyListOrder,
    pub order_direction: OrderDirection,

    pub page: u32,
    pub limit: u32,
}
