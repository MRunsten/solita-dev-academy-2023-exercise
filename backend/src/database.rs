pub mod postgres;

use async_trait::async_trait;

use crate::model::{city, station_operator};
use crate::BoxedError;

pub type DatabaseResult<T> = Result<T, BoxedError>;

#[async_trait]
pub trait Database<T> {
    async fn connect() -> Result<T, BoxedError>;
    async fn initialize(db: &T) -> Result<(), BoxedError>;

    async fn add_city(db: &T, name: city::Name) -> DatabaseResult<city::Id>;
    async fn get_city_by_id(db: &T, id: city::Id) -> DatabaseResult<city::City>;
    async fn get_city_by_name(db: &T, id: city::Name) -> DatabaseResult<city::City>;

    async fn add_station_operator(db: &T, name: station_operator::Name) -> DatabaseResult<station_operator::Id>;
    async fn get_station_operator_by_id(db: &T, name: station_operator::Id) -> DatabaseResult<station_operator::StationOperator>;
    async fn get_station_operator_by_name(db: &T, name: station_operator::Name) -> DatabaseResult<station_operator::StationOperator>;
}
