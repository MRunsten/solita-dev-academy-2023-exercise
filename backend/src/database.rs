mod postgres;
pub mod postgres_view;
pub mod view;

#[cfg(feature = "postgres")]
pub use postgres::*;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
