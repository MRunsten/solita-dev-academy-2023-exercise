use serde::Serialize;
use crate::model::{city, station_operator};
use crate::unit::Coordinate;

pub type Capacity = i32;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct Id(pub i32);

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("{:03}", self.0)
    }
}

impl From<&Id> for i32 {
    fn from(id: &Id) -> Self {
        id.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Name {
    pub finnish: String,
    pub swedish: String,
    pub english: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Address {
    pub finnish: String,
    pub swedish: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Station {
    pub id: Id,

    pub city_id: city::Id,
    pub operator_id: station_operator::Id,

    pub name: Name,
    pub address: Address,
    pub location: Coordinate,
    pub capacity: Capacity,
}
