use std::cmp;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::{database, model};
use crate::api::Pagination;
use crate::database::{Database, DatabaseError};
use crate::database::view::{OrderDirection, StationListOrder, StationListParams};

const STATIONS_MAX_PER_PAGE: u32 = 500;
const DEFAULT_STATIONS_PER_PAGE: u32 = 50;

pub async fn single(
    Path(station_id): Path<i32>,
    State(db): State<Database>
) -> Response {

    match database::view::station(&db, model::station::Id(station_id)).await {
        Ok(station) => {
            return (
                StatusCode::OK,
                Json(station)
            ).into_response()
        },

        Err(err) => {
            tracing::info!("(station_id={station_id}) {err}");

            #[cfg(feature = "postgres")]
            if let DatabaseError::RowNotFound = err {
                return (
                    StatusCode::NOT_FOUND,
                    Json(format!("Station {station_id} was not found from the database"))
                ).into_response()
            }
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(format!("Failed to retrieve station {station_id} from database."))
    ).into_response()
}


pub async fn list(
    pagination: Query<Pagination>,
    State(db): State<Database>,
) -> Response {
    let pagination: Pagination = pagination.0;

    let params = StationListParams {
        order_by: StationListOrder::Id,
        order_direction: OrderDirection::Ascending,
        page: pagination.page.unwrap_or_default(), // Default for u32 is 0.
        limit: cmp::min(
            STATIONS_MAX_PER_PAGE,
            pagination.limit.unwrap_or(DEFAULT_STATIONS_PER_PAGE)
        ),
    };

    match database::view::station_list(&db, &params).await {
        Ok(station_list) => {
            return (
                StatusCode::OK,
                Json(station_list)
            ).into_response()
        },
        Err(err) => {
            tracing::info!("{err}");
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json("Failed to retrieve station list from database.")
    ).into_response()
}
