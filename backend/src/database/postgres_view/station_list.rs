use sqlx::{Execute, Postgres, QueryBuilder, Row};
use crate::database::{Database, DatabaseResult, view};
use crate::model::city;
use crate::station;

pub async fn station_list(
    db: &Database,
    params: &view::StationListParams,
) -> DatabaseResult<Vec<view::StationListItem>> {

    // Warning: The following query uses sqlx::query instead of the sqlx::query! macro. This means
    // that it is not checked during compile time against a database and may lead to runtime issues.
    // This is because of dynamic parameters.
    //
    // Warning: Never edit the following query string to contain arbitrary data (such as user input)
    // as this leads to SQL injections.
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT * FROM station_list_view ORDER BY "
    );

    let mut query_str_tmp = query_builder.separated(" ");
    query_str_tmp.push(params.order_by.to_string());
    query_str_tmp.push(params.order_direction.to_string());

    query_str_tmp.push("LIMIT");
    query_str_tmp.push(params.limit.to_string());

    query_str_tmp.push("OFFSET");
    query_str_tmp.push((params.page * params.limit).to_string());

    let query_str = query_builder.build().sql();

    let rows = sqlx::query(query_str)
        .fetch_all(db)
        .await?;

    let mut station_list_view = Vec::new();

    for row in rows.iter() {
        let station_list_item = view::StationListItem {
            station_id: station::Id(row.try_get("station_id")?),

            name: station::Name {
                finnish: row.try_get("name_finnish")?,
                swedish: row.try_get("name_swedish")?,
                english: row.try_get("name_english")?,
            },

            capacity: row.try_get("capacity")?,

            city_name: city::Name {
                finnish: row.try_get("city_name_finnish")?,
                swedish: row.try_get("city_name_swedish")?,
            },

            operator_name: row.try_get("operator_name")?,
        };

        station_list_view.push(station_list_item);
    }

    Ok(station_list_view)
}
