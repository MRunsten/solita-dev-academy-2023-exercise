use sqlx::PgPool;

use crate::database::DatabaseResult;
use crate::model::journey::JourneyInsert;

pub async fn add_multiple(db: &PgPool, journeys: Vec<JourneyInsert>) -> DatabaseResult<u64> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    for journey in journeys.iter() {
        let tmp_tuple = (
            journey.departure_date,
            journey.departure_station_id.clone(),

            journey.return_date,
            journey.return_station_id.clone(),

            journey.distance,
            journey.duration,
        );

        match csv_writer.serialize(tmp_tuple) {
            Ok(_) => (),
            Err(e) => {
                println!("warning: csv writer could not serialize journeys: {}", e);

                return Ok(0);
            }
        };
    }

    csv_writer.flush()?;

    let mut copy = db.copy_in_raw(r#"
        COPY journeys (
            departure_date,
            departure_station_id,

            return_date,
            return_station_id,

            distance_meters,
            duration_seconds
        )
        FROM STDIN
        WITH (FORMAT CSV)"#,
    ).await?;

    let csv_data = match csv_writer.into_inner() {
        Ok(csv_data) => csv_data,
        Err(e) => {
            println!("warning: csv writer could not create a byte array: {}", e);

            return Ok(0);
        }
    };

    copy.send(csv_data).await?;
    let rows = copy.finish().await?;

    Ok(rows)
}
