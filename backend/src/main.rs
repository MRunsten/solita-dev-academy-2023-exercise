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

    let reinitialize_database =
        env::var("RELOAD_DATABASE").expect("Environment variable RELOAD_DATABASE was undefined");

    match reinitialize_database.as_str() {
        "true" => empty_and_initialize_db(&db).await?,
        "false" => (),
        other => {
            tracing::error!(
                "Invalid environment variable RELOAD_DATABASE='{}', expected=true|false",
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
    tracing::info!("Emptying and reloading database");

    let stations_url = env::var("LOAD_STATIONS_FROM")
        .expect("Environment variable LOAD_STATIONS_FROM was undefined");

    let journey_urls = env::var("LOAD_JOURNEYS_FROM")
        .expect("Environment variable LOAD_JOURNEYS_FROM was undefined");

    database::empty(&db).await?;
    database::initialize(&db).await?;

    tracing::info!("Updating stations");
    let stations_csv = download_url(stations_url.as_str()).await?;

    tracing::info!("Updating stations database");
    let stations_added = datasource::station::csv::update(&db, stations_csv.as_bytes()).await?;
    tracing::info!("Added {stations_added} stations to the database");

    tracing::info!("Updating journeys");
    for journey_url in journey_urls.split(",").collect::<Vec<&str>>().iter() {
        let journey_csv = download_url(journey_url).await?;

        tracing::info!("Updating journeys database");

        let insert_result = datasource::journey::csv::update(&db, journey_csv.as_bytes()).await?;
        let parsed_rows = insert_result.rows_had;
        let unique_new_rows = insert_result.new_rows_inserted;
        let skipped_rows = parsed_rows - unique_new_rows;

        tracing::info!("Parsed {parsed_rows} rows, but added {unique_new_rows} new unique journeys to the database (skipped {skipped_rows} rows).");
    }

    tracing::info!("Refreshing materialized views");
    database::refresh_views(&db).await?;

    tracing::info!("Database reloaded");
    Ok(())
}

async fn download_url(url: &str) -> Result<String, BoxedError> {
    tracing::info!(
        "Downloading {url} (May take a while depending on file size and internet speed)."
    );

    let body = reqwest::get(url).await?.text().await?;
    tracing::info!("Downloading completed.");

    Ok(body)
}
