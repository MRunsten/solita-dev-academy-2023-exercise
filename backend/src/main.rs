mod database;
mod model;
mod unit;

use std::error::Error;
use dotenv::dotenv;

use crate::database::{Database, postgres};

pub type BoxedError = Box<dyn Error>;

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    dotenv().expect("fatal error: .env file not found from the current or parent directory");

    let db = postgres::Postgres::connect().await?;

    Ok(())
}
