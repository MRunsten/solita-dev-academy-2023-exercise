use crate::database::view::JourneyListItemStation;
use crate::database::{view, Database, DatabaseResult};
use crate::station;
use chrono::Utc;
use sqlx::{Execute, Postgres, QueryBuilder, Row};

pub async fn journey_list(
    db: &Database,
    params: &view::JourneyListParams,
) -> DatabaseResult<Vec<view::JourneyListItem>> {
    // Warning: The following query uses sqlx::query instead of the sqlx::query! macro. This means
    // that it is not checked during compile time against a database and may lead to runtime issues.
    // This is because of dynamic parameters.
    //
    // Warning: Never edit the following query string to contain arbitrary data (such as user input)
    // as this leads to SQL injections.
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT * FROM journey_list_view ORDER BY ");

    let mut query_str_tmp = query_builder.separated(" ");
    query_str_tmp.push(params.order_by.to_string());
    query_str_tmp.push(params.order_direction.to_string());

    query_str_tmp.push("LIMIT");
    query_str_tmp.push(params.limit.to_string());

    query_str_tmp.push("OFFSET");
    query_str_tmp.push((params.page * params.limit).to_string());

    let query_str = query_builder.build().sql();

    let rows = sqlx::query(query_str).fetch_all(db).await?;

    let mut journey_list_view = Vec::new();

    for row in rows.iter() {
        let journey_list_item = view::JourneyListItem {
            departure_date: chrono::DateTime::from_utc(row.try_get("departure_date")?, Utc),
            return_date: chrono::DateTime::from_utc(row.try_get("return_date")?, Utc),

            departure_station: JourneyListItemStation {
                station_id: station::Id(row.try_get("departure_station_id")?),
                name: station::Name {
                    finnish: row.try_get("departure_station_name_finnish")?,
                    swedish: row.try_get("departure_station_name_swedish")?,
                    english: row.try_get("departure_station_name_english")?,
                },
            },
            return_station: JourneyListItemStation {
                station_id: station::Id(row.try_get("return_station_id")?),
                name: station::Name {
                    finnish: row.try_get("return_station_name_finnish")?,
                    swedish: row.try_get("return_station_name_swedish")?,
                    english: row.try_get("return_station_name_english")?,
                },
            },
            distance_kilometers: row.try_get("distance_kilometers")?,
            duration_minutes: row.try_get("duration_minutes")?,
        };

        journey_list_view.push(journey_list_item);
    }

    Ok(journey_list_view)
}
