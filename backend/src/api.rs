use std::env;
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use crate::BoxedError;

use crate::database::Database;

pub mod station;
pub mod journey;

pub async fn run(db: Database) -> Result<(), BoxedError> {
    tracing_subscriber::fmt::init();

    let app = define_routes(db);

    let api_address = env::var("API_ADDRESS")
        .expect("Environment variable API_ADDRESS was undefined");

    let addr = api_address.parse()?;
    tracing::info!("Bicycle application backend listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn define_routes(db: Database) -> Router {

    let station_api = Router::new()
        .route("/:id", get(station::single))
        .route("/list", get(station::list));

    let journey_api = Router::new()
        .route("/list", get(journey::list));

    let api = Router::new()
        .nest("/journey", journey_api)
        .nest("/station", station_api);

    let app = Router::new()
        .nest("/api", api)
        .with_state(db);

    app
}
