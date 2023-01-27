pub mod city;
pub mod journey;
pub mod station;
pub mod station_operator;

use sqlx::{Connection, ConnectOptions, PgPool};
use sqlx::postgres::PgConnectOptions;
use std::env;

use crate::database::DatabaseResult;

pub type Database = PgPool;
pub type DatabaseError = sqlx::Error;

pub async fn connect() -> DatabaseResult<Database> {
    let postgres_address =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL was undefined");

    let mut connect_options = postgres_address.parse::<PgConnectOptions>()?;
    connect_options.disable_statement_logging();

    let pool = sqlx::pool::PoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options).await?;

    initialize(&pool).await?;

    Ok(pool)
}
pub async fn initialize(db: &Database) -> DatabaseResult<()> {
    create_tables(db).await?;
    create_views(db).await?;
    create_indices(db).await?;

    refresh_views(db).await?;

    Ok(())
}

pub async fn empty(db: &Database) -> DatabaseResult<()> {
    drop_views(db).await?;
    drop_tables(db).await?;

    Ok(())
}

async fn create_tables(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/01-create_table_cities.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/02-create_table_station_operators.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/03-create_table_stations.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/04-create_table_journeys.sql")
        .execute(db)
        .await?;

    Ok(())
}

async fn create_views(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query_file!("queries/postgres/11-create_view_journey_list.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/12-create_view_station.sql")
        .execute(db)
        .await?;

    let _ = sqlx::query_file!("queries/postgres/13-create_view_station_list.sql")
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

pub async fn refresh_views(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query!("REFRESH MATERIALIZED VIEW journey_list_view").execute(db).await?;
    let _ = sqlx::query!("REFRESH MATERIALIZED VIEW station_list_view").execute(db).await?;
    let _ = sqlx::query!("REFRESH MATERIALIZED VIEW station_view").execute(db).await?;

    Ok(())
}

async fn create_indices(db: &Database) -> DatabaseResult<()> {
    let _ = sqlx::query!("CREATE INDEX IF NOT EXISTS departure_date_index ON journey_list_view(departure_date)")
        .execute(db)
        .await?;

    let _ = sqlx::query!("CREATE INDEX IF NOT EXISTS return_date_index ON journey_list_view(return_date)")
        .execute(db)
        .await?;

    Ok(())
}
