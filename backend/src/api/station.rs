use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::database;
use crate::database::Database;
use crate::database::view::{OrderDirection, StationListOrder, StationListParams};

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
            tracing::trace!("{err}");
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json("Failed to retrieve station list from database.")
    ).into_response()
}
