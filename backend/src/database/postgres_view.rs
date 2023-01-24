mod station;
mod station_list;
mod journey_list;

pub use station::*;
pub use station_list::*;
pub use journey_list::*;

#[derive(Debug)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl ToString for OrderDirection {
    fn to_string(&self) -> String {
        let direction = match self {
            Self::Ascending => "ASC",
            Self::Descending => "DESC",
        };

        direction.to_string()
    }
}

#[derive(Debug)]
pub enum StationListOrder {
    Id,
}

impl ToString for StationListOrder {
    fn to_string(&self) -> String {
        let column = match self {
            Self::Id => "station_id"
        };

        column.to_string()
    }
}

#[derive(Debug)]
pub enum JourneyListOrder {
    DepartureDate,
}

impl ToString for JourneyListOrder {
    fn to_string(&self) -> String {
        let column = match self {
            Self::DepartureDate => "departure_date"
        };

        column.to_string()
    }
}

