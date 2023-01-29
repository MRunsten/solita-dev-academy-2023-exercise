use crate::api::Pagination;
use crate::database::view::{OrderDirection, StationListOrder, StationListParams};
use crate::database::{Database, DatabaseError};
use crate::{database, model};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::cmp;

const STATIONS_MAX_PER_PAGE: u32 = 500;
const DEFAULT_STATIONS_PER_PAGE: u32 = 50;

pub async fn single(Path(station_id): Path<i32>, State(db): State<Database>) -> Response {
    match database::view::station(&db, model::station::Id(station_id)).await {
        Ok(station) => return (StatusCode::OK, Json(station)).into_response(),

        Err(err) => {
            tracing::info!("(station_id={station_id}) {err}");

            #[cfg(feature = "postgres")]
            if let DatabaseError::RowNotFound = err {
                let error_message = format!("Station {station_id} was not found from the database");
                return (StatusCode::NOT_FOUND, Json(error_message)).into_response();
            }
        }
    }

    let error_message = format!("Failed to retrieve station {station_id} from database.");
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
}

pub async fn list(pagination: Query<Pagination>, State(db): State<Database>) -> Response {
    let pagination: Pagination = pagination.0;

    let params = StationListParams {
        order_by: StationListOrder::Id,
        order_direction: OrderDirection::Ascending,
        page: pagination.page.unwrap_or_default(), // Default for u32 is 0.
        limit: cmp::min(
            STATIONS_MAX_PER_PAGE,
            pagination.limit.unwrap_or(DEFAULT_STATIONS_PER_PAGE),
        ),
    };

    match database::view::station_list(&db, &params).await {
        Ok(station_list) => return (StatusCode::OK, Json(station_list)).into_response(),
        Err(err) => {
            tracing::info!("{err}");
        }
    }

    let error_message = "Failed to retrieve station list from database.";
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
}
