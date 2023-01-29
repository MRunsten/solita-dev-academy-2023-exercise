use sqlx::PgPool;

use crate::database::DatabaseResult;
use crate::model::station;
use crate::model::station::Station;
use crate::unit::{Coordinate, Latitude, Longitude};

pub async fn add_multiple(db: &PgPool, stations: Vec<Station>) -> DatabaseResult<u64> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    for station in stations.iter() {
        let tmp_tuple = (
            &station.id,
            &station.city_id,
            &station.operator_id,
            &station.name.finnish,
            &station.name.swedish,
            &station.name.english,
            &station.address.finnish,
            &station.address.swedish,
            f64::from(&station.location.latitude),
            f64::from(&station.location.longitude),
            station.capacity,
        );

        match csv_writer.serialize(tmp_tuple) {
            Ok(_) => (),
            Err(e) => {
                println!("warning: csv writer could not serialize stations: {e}");

                return Ok(0);
            }
        };
    }

    csv_writer.flush()?;

    let mut copy = db
        .copy_in_raw(
            r#"
        COPY stations (
            station_id,

            city_id,
            operator_id,

            name_finnish,
            name_swedish,
            name_english,

            address_finnish,
            address_swedish,

            latitude_north,
            longitude_east,

            capacity
        )
        FROM STDIN
        WITH (FORMAT CSV)"#,
        )
        .await?;

    let csv_data = match csv_writer.into_inner() {
        Ok(csv_data) => csv_data,
        Err(e) => {
            println!("warning: csv writer could not create a byte array: {e}");

            return Ok(0);
        }
    };

    copy.send(csv_data).await?;
    let rows = copy.finish().await?;

    Ok(rows)
}

pub async fn get_all(db: &PgPool) -> DatabaseResult<Vec<Station>> {
    let stations = sqlx::query!(
        r#"
        SELECT

        station_id,

        city_id,
        operator_id,

        name_finnish,
        name_swedish,
        name_english,

        address_finnish,
        address_swedish,

        latitude_north,
        longitude_east,

        capacity

        FROM stations
        "#,
    )
    .map(|record| Station {
        id: station::Id(record.station_id),

        city_id: record.city_id,
        operator_id: record.operator_id,

        name: station::Name {
            finnish: record.name_finnish,
            swedish: record.name_swedish,
            english: record.name_english,
        },

        address: station::Address {
            finnish: record.address_finnish,
            swedish: record.address_swedish,
        },

        location: Coordinate {
            latitude: Latitude::North(record.latitude_north),
            longitude: Longitude::East(record.longitude_east),
        },

        capacity: record.capacity,
    })
    .fetch_all(db)
    .await?;

    Ok(stations)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::database::DatabaseResult;
    use crate::model::station::Station;
    use crate::model::{city, station, station_operator};
    use crate::unit::{Coordinate, Latitude, Longitude};

    // The following function is currently only used during tests, but it can be pulled
    // from the tests module if the actual application needs the get_by_id() function
    // in the future.
    pub async fn get_by_id(db: &PgPool, station_id: station::Id) -> DatabaseResult<Station> {
        let record = sqlx::query!(
            r#"
        SELECT

        city_id,
        operator_id,

        name_finnish,
        name_swedish,
        name_english,

        address_finnish,
        address_swedish,

        latitude_north,
        longitude_east,

        capacity

        FROM stations
        WHERE station_id = $1
        "#,
            i32::from(&station_id),
        )
        .fetch_one(db)
        .await?;

        let station = Station {
            id: station_id,

            city_id: record.city_id,
            operator_id: record.operator_id,

            name: station::Name {
                finnish: record.name_finnish,
                swedish: record.name_swedish,
                english: record.name_english,
            },

            address: station::Address {
                finnish: record.address_finnish,
                swedish: record.address_swedish,
            },

            location: Coordinate {
                latitude: Latitude::North(record.latitude_north),
                longitude: Longitude::East(record.longitude_east),
            },

            capacity: record.capacity,
        };

        Ok(station)
    }

    #[sqlx::test]
    async fn test_station(db: PgPool) -> DatabaseResult<()> {
        use crate::database;

        database::initialize(&db).await?;

        let city_name = city::Name {
            finnish: "city name in finnish".to_string(),
            swedish: "city name in swedish".to_string(),
        };

        let operator_name = "station_operator's name".to_string();

        let city_id = database::city::add(&db, city_name.clone()).await?;
        let operator_id = database::station_operator::add(&db, operator_name.clone()).await?;

        let station_id42 = station::Id(42);
        let station_id123 = station::Id(123);

        let station42 =
            get_mock_station(station_id42.clone(), city_id.clone(), operator_id.clone());
        let station123 =
            get_mock_station(station_id123.clone(), city_id.clone(), operator_id.clone());

        let rows =
            database::station::add_multiple(&db, vec![station42.clone(), station123.clone()])
                .await?;

        assert_eq!(rows, 2);

        let station42_by_id = get_by_id(&db, station_id42.clone()).await?;
        let station123_by_id = get_by_id(&db, station_id123.clone()).await?;

        assert_eq!(station42, station42_by_id);
        assert_eq!(station123, station123_by_id);

        database::empty(&db).await?;

        let station42_by_id_result = get_by_id(&db, station_id42).await;
        let station123_by_id_result = get_by_id(&db, station_id123).await;

        assert!(station42_by_id_result.is_err());
        assert!(station123_by_id_result.is_err());

        Ok(())
    }

    fn get_mock_station(
        station_id: station::Id,
        city_id: city::Id,
        operator_id: station_operator::Id,
    ) -> Station {
        Station {
            id: station_id.clone(),

            city_id,
            operator_id,

            name: station::Name {
                finnish: format!("station {station_id:?} name in Finnish"),
                swedish: format!("station {station_id:?} name in Swedish"),
                english: format!("station {station_id:?} name in English"),
            },

            address: station::Address {
                finnish: format!("station {station_id:?} address in Finnish"),
                swedish: format!("station {station_id:?} address in Swedish"),
            },

            location: Coordinate {
                latitude: Latitude::North(24.819396),
                longitude: Longitude::East(60.216067),
            },

            capacity: 42,
        }
    }
}
