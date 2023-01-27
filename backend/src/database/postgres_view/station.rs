use crate::database::{view, Database, DatabaseResult};
use crate::model::{city, station, station_operator};

pub async fn station(db: &Database, station_id: station::Id) -> DatabaseResult<view::Station> {
    let record = sqlx::query!(
        "SELECT * FROM station_view WHERE station_id = $1",
        i32::from(&station_id),
    )
    .fetch_one(db)
    .await?;

    // Note: The existence on the following columns is checked by sqlx during compile time,
    // but the record type returns Option<T> values. This is because views in Postgres technically
    // can contain NULL values and it requires quite a bit of trickery in order to never return
    // NULL values from a view.
    //
    // This view should never return NULL (None) values.
    let station_view = view::Station {
        station_id,
        name: station::Name {
            finnish: record.name_finnish.unwrap_or("error".to_string()),
            swedish: record.name_swedish.unwrap_or("error".to_string()),
            english: record.name_english.unwrap_or("error".to_string()),
        },

        address: station::Address {
            finnish: record.address_finnish.unwrap_or("error".to_string()),
            swedish: record.address_swedish.unwrap_or("error".to_string()),
        },

        city: city::Name {
            finnish: record.city_name_finnish.unwrap_or("error".to_string()),
            swedish: record.city_name_swedish.unwrap_or("error".to_string()),
        },

        operator_name: record.operator_name.unwrap_or("error".to_string()),

        total_starting_journeys: record.journeys_departing_amount.unwrap_or(0) as u64,
        total_ending_journeys: record.journeys_returning_amount.unwrap_or(0) as u64,
    };

    Ok(station_view)
}
