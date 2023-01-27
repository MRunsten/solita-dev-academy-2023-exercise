mod postgres;
pub mod postgres_view;
pub mod view;

#[cfg(feature = "postgres")]
pub use postgres::*;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

pub struct JourneyInsertResult {
    pub rows_had: u64,
    pub new_rows_inserted: u64,
}
