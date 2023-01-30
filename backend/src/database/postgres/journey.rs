use sqlx::PgPool;

use crate::database::{DatabaseResult, JourneyInsertResult};
use crate::model::journey::JourneyInsert;

pub async fn add_multiple(
    db: &PgPool,
    journeys: Vec<JourneyInsert>,
) -> DatabaseResult<JourneyInsertResult> {
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
                println!("warning: csv writer could not serialize journeys: {e}");

                return Ok(JourneyInsertResult {
                    rows_had: journeys.len() as u64,
                    new_rows_inserted: 0,
                });
            }
        };
    }

    csv_writer.flush()?;

    let csv_data = match csv_writer.into_inner() {
        Ok(csv_data) => csv_data,
        Err(e) => {
            println!("warning: csv writer could not create a byte array: {e}");

            return Ok(JourneyInsertResult {
                rows_had: journeys.len() as u64,
                new_rows_inserted: 0,
            });
        }
    };

    let mut tx = db.begin().await?;

    sqlx::query!(
        "CREATE TEMP TABLE tmp_journeys (LIKE journeys INCLUDING DEFAULTS) ON COMMIT DROP"
    )
    .execute(&mut tx)
    .await?;

    let mut copy = tx
        .copy_in_raw(
            r#"
        COPY tmp_journeys (
            departure_date,
            departure_station_id,

            return_date,
            return_station_id,

            distance_meters,
            duration_seconds
        )
        FROM STDIN
        WITH (FORMAT CSV)"#,
        )
        .await?;

    copy.send(csv_data).await?;
    let rows = copy.finish().await?;

    let insert_result =
        sqlx::query("INSERT INTO journeys SELECT * FROM tmp_journeys ON CONFLICT DO NOTHING")
            .execute(&mut tx)
            .await?;

    tx.commit().await?;

    Ok(JourneyInsertResult {
        rows_had: rows,
        new_rows_inserted: insert_result.rows_affected(),
    })
}
