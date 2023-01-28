use crate::database;
use crate::database::{Database, DatabaseResult, JourneyInsertResult};
use crate::datasource::DataSourceResult;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;
use crate::model::journey::JourneyInsert;
use crate::model::station;

use crate::unit::{Meters, Seconds};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PossibleDateTypes {
    OnlyDate(NaiveDate),
    WithTime(NaiveDateTime)
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
        // .quote_style(csv::QuoteStyle::Necessary)
        // .flexible(true)
        .from_reader(source)
        .deserialize()
        .collect::<Vec<Result<CsvBicycleJourney, csv::Error>>>();

    let valid_stations = get_valid_stations_ids(&db).await?;

    let mut parsed_journeys = Vec::new();

    for maybe_csv_journey in csv_journeys.into_iter() {
        if let Err(err) = maybe_csv_journey {
            tracing::warn!("(skipping csv row): {err}");
            continue
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

    Ok(database::journey::add_multiple(&db, parsed_journeys).await?)
}

async fn get_valid_stations_ids(
    db: &Database,
) -> DatabaseResult<HashSet<station::Id>> {

    let valid_stations = database::station::get_all(&db).await?.into_iter().map(|station| station.id);

    Ok(HashSet::from_iter(valid_stations))
}
