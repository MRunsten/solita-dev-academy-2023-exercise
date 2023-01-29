use crate::database;
use crate::database::{Database, DatabaseResult, JourneyInsertResult};
use crate::datasource::DataSourceResult;
use crate::model::journey::JourneyInsert;
use crate::model::station;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;

use crate::unit::{Meters, Seconds};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PossibleDateTypes {
    OnlyDate(NaiveDate),
    WithTime(NaiveDateTime),
}

#[derive(Deserialize, Debug)]
pub struct CsvBicycleJourney {
    #[serde(rename = "Departure")]
    departure_date: PossibleDateTypes,

    #[serde(rename = "Return")]
    return_date: PossibleDateTypes,

    #[serde(rename = "Departure station id")]
    departure_station_id: i32,
    // Note: Field "Departure station name" is unused.
    #[serde(rename = "Return station id")]
    return_station_id: i32,
    // Note: Field "Return station name" is unused.
    #[serde(rename = "Covered distance (m)")]
    covered_distance: Option<Meters>,

    #[serde(rename = "Duration (sec.)")]
    duration: Seconds,
}

pub async fn update<Source>(db: &Database, source: Source) -> DataSourceResult<JourneyInsertResult>
where
    Source: Read + Sync + Send,
{
    let csv_journeys = csv::ReaderBuilder::new()
        .from_reader(source)
        .deserialize()
        .collect::<Vec<Result<CsvBicycleJourney, csv::Error>>>();

    let valid_stations = get_valid_stations_ids(db).await?;

    let mut parsed_journeys = Vec::new();

    for maybe_csv_journey in csv_journeys.into_iter() {
        if let Err(err) = maybe_csv_journey {
            tracing::warn!("(skipping csv row): {err}");
            continue;
        }

        let csv_journey = maybe_csv_journey.unwrap();

        let departure_station_id = station::Id(csv_journey.departure_station_id);
        let return_station_id = station::Id(csv_journey.return_station_id);

        if !&valid_stations.contains(&departure_station_id) {
            continue;
        }

        if !&valid_stations.contains(&return_station_id) {
            continue;
        }

        let covered_distance = match csv_journey.covered_distance {
            Some(distance) => distance,
            None => {
                continue;
            }
        };

        let departure_date = match csv_journey.departure_date {
            PossibleDateTypes::OnlyDate(date) => date.and_hms_opt(0, 0, 0).unwrap(),
            PossibleDateTypes::WithTime(datetime) => datetime,
        };

        let return_date = match csv_journey.return_date {
            PossibleDateTypes::OnlyDate(date) => date.and_hms_opt(0, 0, 0).unwrap(),
            PossibleDateTypes::WithTime(datetime) => datetime,
        };

        let journey_insert = JourneyInsert {
            departure_date,
            departure_station_id,

            return_date,
            return_station_id,

            distance: covered_distance,
            duration: csv_journey.duration,
        };

        if journey_insert.is_valid() {
            parsed_journeys.push(journey_insert);
        }
    }

    Ok(database::journey::add_multiple(db, parsed_journeys).await?)
}

async fn get_valid_stations_ids(db: &Database) -> DatabaseResult<HashSet<station::Id>> {
    let valid_stations = database::station::get_all(db)
        .await?
        .into_iter()
        .map(|station| station.id);

    Ok(HashSet::from_iter(valid_stations))
}

#[cfg(test)]
mod tests {
    use crate::database;
    use crate::database::view::{JourneyListOrder, JourneyListParams, OrderDirection};
    use crate::datasource;
    use crate::model;
    use crate::BoxedError;
    use sqlx::PgPool;
    use std::fs::File;

    #[sqlx::test]
    async fn disallow_less_than_10_second(db: PgPool) -> Result<(), BoxedError> {
        use std::fs::File;

        database::initialize(&db).await?;

        let station_test_data = File::open("./tests/pipeline_test_data/station_pipeline.csv")?;
        let stations_added = datasource::station::csv::update(&db, station_test_data).await?;
        assert_eq!(stations_added, 2);

        let journey_test_data =
            File::open("./tests/pipeline_test_data/journey_disallows_less_than_10_second.csv")?;

        let journeys_added = datasource::journey::csv::update(&db, journey_test_data).await?;
        assert_eq!(journeys_added.rows_had, 1);
        assert_eq!(journeys_added.new_rows_inserted, 1);

        database::refresh_views(&db).await?;

        let params = JourneyListParams {
            order_by: JourneyListOrder::DepartureDate,
            order_direction: OrderDirection::Descending,
            page: 0,
            limit: 100,
        };

        let journeys = database::view::journey_list(&db, &params).await?;
        assert_eq!(journeys.len(), 1);

        journeys.iter().for_each(|j| {
            assert!(j.duration_minutes >= 0.16);
        });

        Ok(())
    }

    #[sqlx::test]
    async fn disallow_less_than_10_meter(db: PgPool) -> Result<(), BoxedError> {
        database::initialize(&db).await?;

        let station_test_data = File::open("./tests/pipeline_test_data/station_pipeline.csv")?;
        let stations_added = datasource::station::csv::update(&db, station_test_data).await?;
        assert_eq!(stations_added, 2);

        let journey_test_data =
            File::open("./tests/pipeline_test_data/journey_disallows_less_than_10_meter.csv")?;

        let journeys_added = datasource::journey::csv::update(&db, journey_test_data).await?;
        assert_eq!(journeys_added.rows_had, 1);
        assert_eq!(journeys_added.new_rows_inserted, 1);

        database::refresh_views(&db).await?;

        let params = JourneyListParams {
            order_by: JourneyListOrder::DepartureDate,
            order_direction: OrderDirection::Descending,
            page: 0,
            limit: 100,
        };

        let journeys = database::view::journey_list(&db, &params).await?;
        assert_eq!(journeys.len(), 1);

        journeys
            .iter()
            .for_each(|j| assert!(j.distance_kilometers >= 0.0099));

        Ok(())
    }

    #[sqlx::test]
    async fn disallow_invalid_stations(db: PgPool) -> Result<(), BoxedError> {
        database::initialize(&db).await?;

        let station_test_data = File::open("./tests/pipeline_test_data/station_pipeline.csv")?;
        let stations_added = datasource::station::csv::update(&db, station_test_data).await?;
        assert_eq!(stations_added, 2);

        let journey_test_data =
            File::open("./tests/pipeline_test_data/journey_disallows_invalid_stations.csv")?;

        let journeys_added = datasource::journey::csv::update(&db, journey_test_data).await?;
        assert_eq!(journeys_added.rows_had, 1);
        assert_eq!(journeys_added.new_rows_inserted, 1);

        database::refresh_views(&db).await?;

        let params = JourneyListParams {
            order_by: JourneyListOrder::DepartureDate,
            order_direction: OrderDirection::Descending,
            page: 0,
            limit: 100,
        };

        let journeys = database::view::journey_list(&db, &params).await?;
        assert_eq!(journeys.len(), 1);

        journeys.iter().for_each(|j| {
            assert_eq!(j.departure_station.station_id, model::station::Id(42));
            assert_eq!(j.return_station.station_id, model::station::Id(123));
        });

        Ok(())
    }

    #[sqlx::test]
    async fn disallow_duplicates(db: PgPool) -> Result<(), BoxedError> {
        database::initialize(&db).await?;

        let station_test_data = File::open("./tests/pipeline_test_data/station_pipeline.csv")?;
        let stations_added = datasource::station::csv::update(&db, station_test_data).await?;
        assert_eq!(stations_added, 2);

        let journey_test_data =
            File::open("./tests/pipeline_test_data/journey_disallows_duplicates.csv")?;
        let journeys_added = datasource::journey::csv::update(&db, journey_test_data).await?;
        assert_eq!(journeys_added.rows_had, 3);
        assert_eq!(journeys_added.new_rows_inserted, 1);

        Ok(())
    }
}
