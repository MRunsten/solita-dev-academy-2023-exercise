use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use std::cmp;

use crate::database;
use crate::database::view::{JourneyListOrder, JourneyListParams, OrderDirection};
use crate::database::Database;

const LIST_MAX_PER_PAGE: u32 = 100;
const DEFAULT_LIST_PER_PAGE: u32 = 100;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

pub async fn list(pagination: Query<Pagination>, State(db): State<Database>) -> Response {
    let pagination: Pagination = pagination.0;

    let params = JourneyListParams {
        order_by: JourneyListOrder::DepartureDate,
        order_direction: OrderDirection::Descending,
        page: pagination.page.unwrap_or_default(), // Default for u32 is 0.
        limit: cmp::min(
            LIST_MAX_PER_PAGE,
            pagination.limit.unwrap_or(DEFAULT_LIST_PER_PAGE)
        ),
    };

    match database::view::journey_list(&db, &params).await {
        Ok(journey_list) => {
            return (StatusCode::OK, Json(journey_list)).into_response()
        },
        Err(err) => {
            tracing::info!("({:?}) {err}", params);
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json("Failed to retrieve journey list from database."),
    ).into_response()
}
