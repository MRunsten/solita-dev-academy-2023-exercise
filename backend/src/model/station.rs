use crate::model::{city, station_operator};
use crate::unit::Coordinate;

#[derive(Debug, Clone)]
pub struct Id(i32);

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("{:03}", self.0)
    }
}

pub type Capacity = i32;

#[derive(Debug, Clone)]
pub struct Name {
    pub finnish: String,
    pub swedish: String,
    pub english: String,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub finnish: String,
    pub swedish: String,
}

#[derive(Debug, Clone)]
pub struct Station {
    pub id: Id,

    pub city_id: city::Id,
    pub operator_id: station_operator::Id,

    pub name: Name,
    pub address: Address,
    pub location: Coordinate,
    pub capacity: Capacity,
}
