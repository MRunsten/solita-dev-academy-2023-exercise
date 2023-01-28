use crate::database;
use crate::database::{Database, DatabaseResult};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::Read;

use crate::datasource::DataSourceResult;
use crate::model::station::Station;
use crate::model::{city, station, station_operator};
use crate::unit::{Coordinate, Latitude, Longitude};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CsvBicycleStation {
    #[serde(rename = "ID")]
    id: i32,

    nimi: String,
    namn: String,
    name: String,

    osoite: String,
    adress: String,

    kaupunki: String,
    stad: String,

    operaattor: String,
    kapasiteet: i32,

    #[serde(rename = "x")]
    x: f64,

    #[serde(rename = "y")]
    y: f64,
}

pub async fn update<Source>(db: &Database, source: Source) -> DataSourceResult<u64>
where
    Source: Read + Sync + Send,
{
    let csv_stations = csv::Reader::from_reader(source)
        .deserialize()
        .collect::<Result<Vec<CsvBicycleStation>, csv::Error>>()?;

    let city_to_id = get_city_name_to_id_cache(db, &csv_stations).await?;
    let operator_to_id = get_operator_name_to_id_cache(db, &csv_stations).await?;

    let mut parsed_stations = Vec::new();

    for csv_station in csv_stations.into_iter() {
        let station = Station {
            id: station::Id(csv_station.id),

            city_id: *city_to_id
                .get(&city::Name {
                    finnish: csv_station.kaupunki,
                    swedish: csv_station.stad,
                })
                .unwrap(), // Note: All unique cities are within the hashmap or an error has been returned.

            operator_id: *operator_to_id
                .get(&csv_station.operaattor)
                .unwrap(), // Note: All unique operators are within the hashmap or an error has been returned.

            name: station::Name {
                finnish: csv_station.nimi,
                swedish: csv_station.namn,
                english: csv_station.name,
            },

            address: station::Address {
                finnish: csv_station.osoite,
                swedish: csv_station.adress,
            },

            capacity: csv_station.kapasiteet,

            // Note: y before x is correct in this case.
            location: Coordinate::new(
                Latitude::North(csv_station.y),
                Longitude::East(csv_station.x),
            ),
        };

        parsed_stations.push(station);
    }

    Ok(database::station::add_multiple(db, parsed_stations).await?)
}

async fn get_city_name_to_id_cache(
    db: &Database,
    stations: &Vec<CsvBicycleStation>,
) -> DatabaseResult<HashMap<city::Name, city::Id>> {
    let mut city_name_to_id = HashMap::new();

    let unique_cities = stations.iter().fold(HashSet::new(), |mut acc, station| {
        acc.insert((station.kaupunki.clone(), station.stad.clone()));
        acc
    });

    for (name_finnish, name_swedish) in unique_cities.iter() {
        let city_name = city::Name {
            finnish: name_finnish.clone(),
            swedish: name_swedish.clone(),
        };

        let city_id = database::city::get_or_add_by_name(db, city_name.clone())
            .await?
            .id;

        city_name_to_id.insert(city_name, city_id);
    }

    Ok(city_name_to_id)
}

async fn get_operator_name_to_id_cache(
    db: &Database,
    stations: &Vec<CsvBicycleStation>,
) -> DatabaseResult<HashMap<station_operator::Name, station_operator::Id>> {
    let mut operator_name_to_id = HashMap::new();

    let unique_operators = stations.iter().fold(
        HashSet::<station_operator::Name>::new(),
        |mut acc, station| {
            acc.insert(station.operaattor.clone());
            acc
        },
    );

    for operator_name in unique_operators.iter() {
        let operator_id =
            database::station_operator::get_or_add_by_name(db, operator_name.clone())
                .await?
                .id;

        operator_name_to_id.insert(operator_name.clone(), operator_id);
    }

    Ok(operator_name_to_id)
}
