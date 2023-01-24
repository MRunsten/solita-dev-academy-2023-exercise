use crate::BoxedError;

pub mod journey;
pub mod station;

pub type DataSourceError = BoxedError;
pub type DataSourceResult<T> = Result<T, DataSourceError>;
