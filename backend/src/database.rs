mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::*;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
