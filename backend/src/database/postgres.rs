use std::env;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::BoxedError;
use crate::database::Database;

pub struct Postgres;

#[async_trait]
impl Database<PgPool> for Postgres {
    async fn connect() -> Result<PgPool, BoxedError> {
        let pg_address = env::var("DATABASE_URL")
            .expect("Environment variable DATABASE_URL was undefined");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*pg_address)
            .await?;

        Ok(pool)
    }
}
