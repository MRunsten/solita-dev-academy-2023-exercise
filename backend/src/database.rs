pub mod postgres;

use async_trait::async_trait;

use crate::BoxedError;

#[async_trait]
pub trait Database<T> {
    async fn connect() -> Result<T, BoxedError>;
}
