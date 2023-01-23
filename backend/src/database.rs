pub mod postgres;

use async_trait::async_trait;

use crate::BoxedError;
use crate::model::city;

#[async_trait]
pub trait Database<T> {
    async fn connect() -> Result<T, BoxedError>;
    async fn initialize(db: &T) -> Result<(), BoxedError>;

    async fn add_city(db: &T, name: city::Name) -> Result<city::Id, BoxedError>;
    async fn get_city_by_id(db: &T, id: city::Id) -> Result<city::City, BoxedError>;
    async fn get_city_by_name(db: &T, id: city::Name) -> Result<city::City, BoxedError>;
}
