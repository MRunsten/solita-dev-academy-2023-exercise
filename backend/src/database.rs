mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::*;

use crate::BoxedError;

pub type DatabaseResult<T> = Result<T, BoxedError>;
