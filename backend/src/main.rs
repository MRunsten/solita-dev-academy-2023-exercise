mod api;
mod database;
mod datasource;
mod model;
mod unit;

use crate::database::Database;
use crate::model::station;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::io::Read;

pub type BoxedError = Box<dyn Error>;

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    tracing_subscriber::fmt::init();

    dotenv().expect("fatal error: .env file not found from the current or parent directory");

    let db = database::connect().await?;

    let reinitialize_database = env::var("REINITIALIZE_DATABASE")
        .expect("Environment variable REINITIALIZE_DATABASE was undefined");

    match reinitialize_database.as_str() {
        "true" => empty_and_initialize_db(&db).await?,
        "false" => (),
        other => {
            tracing::error!(
                "Invalid environment variable REINITIALIZE_DATABASE='{}', expected=true|false",
                other
            );
        }
    }

    let api_run = env::var("API_RUN").expect("Environment variable API_RUN was undefined");

    match api_run.as_str() {
        "true" => api::run(db).await?,
        "false" => (),
        other => {
            tracing::error!(
                "Invalid environment variable API_RUN='{}', expected=true|false",
                other
            );
        }
    }

    Ok(())
}

async fn empty_and_initialize_db(db: &Database) -> Result<(), BoxedError> {
    tracing::info!("Emptying and reinitializing database");

    let stations_url = env::var("LOAD_STATIONS_FROM")
        .expect("Environment variable LOAD_STATIONS_FROM was undefined");

    let journey_urls = env::var("LOAD_JOURNEYS_FROM")
        .expect("Environment variable LOAD_JOURNEYS_FROM was undefined");

    database::empty(&db).await?;
    database::initialize(&db).await?;

    tracing::info!("Updating stations");
    let stations_csv = download_url(stations_url.as_str()).await?;
    datasource::station::csv::update(&db, stations_csv.as_bytes()).await?;

    tracing::info!("Updating journeys");
    for journey_url in journey_urls.split(",").collect::<Vec<&str>>().iter() {
        let journey_csv = download_url(journey_url).await?;
        datasource::journey::csv::update(&db, journey_csv.as_bytes()).await?;
    }

    tracing::info!("Database reinitialized");
    Ok(())
}

async fn download_url(url: &str) -> Result<String, BoxedError> {
    tracing::info!("Downloading {url} (May take a while depending on file size and internet speed).");
    Ok(reqwest::get(url).await?.text().await?)
}
