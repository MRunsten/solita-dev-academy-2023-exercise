pub mod city;
pub mod station;
pub mod station_operator;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::database::DatabaseResult;

pub type Database = PgPool;
pub type DatabaseError = sqlx::Error;

pub async fn connect() -> DatabaseResult<Database> {
    let postgres_address =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL was undefined");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*postgres_address)
        .await?;

    initialize(&pool).await?;

    Ok(pool)
}

pub async fn initialize(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/create_table_cities.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_table_station_operators.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_table_stations.sql")
        .execute(db)
        .await?;

    Ok(())
}

pub async fn empty(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/drop_table_stations.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/drop_table_station_operators.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/drop_table_cities.sql")
        .execute(db)
        .await?;

    Ok(())
}
