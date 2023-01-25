use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::{database, model};
use crate::database::{Database, DatabaseError};
use crate::database::view::{OrderDirection, StationListOrder, StationListParams};

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
                    Json(format!("station {station_id} was not found the database"))
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
    State(db): State<Database>
) -> Response {

    let params = StationListParams {
        order_by: StationListOrder::Id,
        order_direction: OrderDirection::Ascending
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
