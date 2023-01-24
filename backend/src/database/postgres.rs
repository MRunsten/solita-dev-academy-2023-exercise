pub mod city;
pub mod journey;
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

    // Unfortunately at least Postgres doesn't support conditional
    // view generation (..IF NOT EXISTS, ... OR REPLACE). However,
    // views are quite fast to re-create.
    drop_views(db).await?;

    create_tables(db).await?;
    create_views(db).await?;

    Ok(())
}

pub async fn empty(db: &Database) -> DatabaseResult<()> {
    drop_views(db).await?;
    drop_tables(db).await?;

    Ok(())
}

async fn create_tables(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/create_table_cities.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_table_station_operators.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_table_stations.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_table_journeys.sql")
        .execute(db)
        .await?;

    Ok(())
}

async fn create_views(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/create_view_station.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_view_station_list.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/create_view_journey_list.sql")
        .execute(db)
        .await?;

    Ok(())
}

async fn drop_tables(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/drop_table_journeys.sql")
        .execute(db)
        .await?;

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

async fn drop_views(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/drop_view_station.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/drop_view_station_list.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/drop_view_journey_list.sql")
        .execute(db)
        .await?;

    Ok(())
}
