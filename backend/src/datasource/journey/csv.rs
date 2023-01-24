use crate::database;
use crate::database::{Database, DatabaseResult};
use crate::datasource::DataSourceResult;
use crate::model::journey::{Journey, JourneyInsert};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::Read;
use crate::model::station;

use crate::unit::{Meters, Seconds};

#[derive(Deserialize, Debug)]
pub struct CsvBicycleJourney {
    #[serde(rename = "Departure")]
    departure_date: NaiveDateTime,

    #[serde(rename = "Return")]
    return_date: NaiveDateTime,

    #[serde(rename = "Departure station id")]
    departure_station_id: i32,
    // Note: Field "Departure station name" is unused.
    #[serde(rename = "Return station id")]
    return_station_id: i32,
    // Note: Field "Return station name" is unused.
    #[serde(rename = "Covered distance (m)")]
    covered_distance: Meters,

    #[serde(rename = "Duration (sec.)")]
    duration: Seconds,
}

pub async fn update<Source>(db: &Database, source: Source) -> DataSourceResult<()>
where
    Source: Read + Sync + Send,
{
    use std::time::Instant;
    let now = Instant::now();

    let csv_journeys = csv::Reader::from_reader(source)
        .deserialize()
        .collect::<Result<Vec<CsvBicycleJourney>, csv::Error>>()?;

    let mut valid_stations = get_valid_stations_ids(&db, &csv_journeys).await?;

    dbg!(valid_stations.len());

    let mut parsed_journeys = Vec::new();

    for csv_journey in csv_journeys.into_iter() {
        let departure_station_id = station::Id(csv_journey.departure_station_id);
        let return_station_id = station::Id(csv_journey.return_station_id);

        if !&valid_stations.contains(&departure_station_id) {
            continue;
        }

        if !&valid_stations.contains(&return_station_id) {
            continue;
        }


        let journey_insert = JourneyInsert {
            departure_date: csv_journey.departure_date,
            departure_station_id,

            return_date: csv_journey.return_date,
            return_station_id,

            distance: csv_journey.covered_distance,
            duration: csv_journey.duration,
        };

        if journey_insert.is_valid() {
            parsed_journeys.push(journey_insert);
        }
    }

    dbg!(parsed_journeys.len());

    database::journey::add_multiple(&db, parsed_journeys).await?;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

async fn get_valid_stations_ids(
    db: &Database,
    csv_journeys: &Vec<CsvBicycleJourney>,
) -> DatabaseResult<HashSet<station::Id>> {
    // let valid_stations = HashSet::new();

    // let mut unique_stations =
    //     csv_journeys
    //         .iter()
    //         .fold(HashSet::<station::Id>::new(), |mut acc, journey| {
    //             acc.insert(station::Id(journey.departure_station_id));
    //             acc.insert(station::Id(journey.return_station_id));
    //             acc
    //         });
    //
    // dbg!(unique_stations);

    let valid_stations = database::station::get_all(&db).await?.into_iter().map(|station| station.id);

    // dbg!(stations.len());

    Ok(HashSet::from_iter(valid_stations))
}
