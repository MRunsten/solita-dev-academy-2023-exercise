use serde::Serialize;

pub type Meters = f64;
pub type Kilometers = f64;

pub type Seconds = f64;
pub type Minutes = f64;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Latitude {
    North(f64),
    South(f64),
}

impl From<&Latitude> for f64 {
    fn from(value: &Latitude) -> Self {
        *match value {
            Latitude::North(gps_coordinate) => gps_coordinate,
            Latitude::South(gps_coordinate) => gps_coordinate
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Longitude {
    East(f64),
    West(f64),
}

impl From<&Longitude> for f64 {
    fn from(value: &Longitude) -> Self {
        *match value {
            Longitude::East(gps_coordinate) => gps_coordinate,
            Longitude::West(gps_coordinate) => gps_coordinate
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Coordinate {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl Coordinate {
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Coordinate {
            latitude,
            longitude,
        }
    }
}
