pub mod city;
pub mod station_operator;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

use crate::database::{Database, DatabaseResult};

pub struct Postgres;

#[async_trait]
impl Database<PgPool> for Postgres {
    async fn connect() -> DatabaseResult<PgPool> {
        let postgres_address =
            env::var("DATABASE_URL").expect("Environment variable DATABASE_URL was undefined");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*postgres_address)
            .await?;

        Postgres::initialize(&pool).await?;

        Ok(pool)
    }

    async fn initialize(db: &PgPool) -> DatabaseResult<()> {
        let _ = sqlx::query_file!("queries/postgres/create_table_cities.sql")
            .execute(db)
            .await?;

        let _ = sqlx::query_file!("queries/postgres/create_table_station_operators.sql")
            .execute(db)
            .await?;

        Ok(())
    }
}

