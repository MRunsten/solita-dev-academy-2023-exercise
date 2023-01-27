use crate::BoxedError;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::env;
use axum::http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::database::Database;

pub mod journey;
pub mod station;

pub async fn run(db: Database) -> Result<(), BoxedError> {
    let app = define_routes(db);

    let api_address =
        env::var("API_ADDRESS").expect("Environment variable API_ADDRESS was undefined");

    let addr = api_address.parse()?;
    tracing::info!("Bicycle application backend listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn define_routes(db: Database) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let station_api = Router::new()
        .route("/:id", get(station::single));

    let stations_api = Router::new()
        .route("/", get(station::list));

    let journeys_api = Router::new().route("/", get(journey::list));

    let api = Router::new()
        .nest("/journeys", journeys_api)
        .nest("/station", station_api)
        .nest("/stations", stations_api)
        .layer(ServiceBuilder::new().layer(cors));

    let app = Router::new().nest("/api", api).with_state(db);

    app
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}
