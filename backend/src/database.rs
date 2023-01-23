pub mod postgres;

use async_trait::async_trait;

use crate::model::city;
use crate::BoxedError;

pub type DatabaseResult<T> = Result<T, BoxedError>;

#[async_trait]
pub trait Database<T> {
    async fn connect() -> Result<T, BoxedError>;
    async fn initialize(db: &T) -> Result<(), BoxedError>;

    async fn add_city(db: &T, name: city::Name) -> DatabaseResult<city::Id>;
    async fn get_city_by_id(db: &T, id: city::Id) -> DatabaseResult<city::City>;
    async fn get_city_by_name(db: &T, id: city::Name) -> DatabaseResult<city::City>;
}
