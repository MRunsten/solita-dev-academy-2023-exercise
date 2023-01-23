use async_trait::async_trait;
use sqlx::PgPool;
use crate::Database;
use crate::database::{DatabaseResult, StationOperatorDatabase};
use crate::model::station_operator;
use crate::model::station_operator::StationOperator;
use crate::postgres::Postgres;

#[async_trait]
impl StationOperatorDatabase<PgPool> for Postgres {
    async fn add_station_operator(db: &PgPool, name: station_operator::Name) -> DatabaseResult<station_operator::Id> {
        let operator_id = sqlx::query!(
            "INSERT INTO station_operators (operator_name) VALUES ($1) RETURNING operator_id",
            &name,
        )
            .fetch_one(db)
            .await?
            .operator_id;

        Ok(operator_id)
    }

    async fn get_station_operator_by_id(db: &PgPool, operator_id: station_operator::Id) -> DatabaseResult<StationOperator> {
        let record = sqlx::query!(
            "SELECT operator_name FROM station_operators WHERE operator_id = $1",
            &operator_id
        )
            .fetch_one(db)
            .await?;

        Ok(StationOperator {
            id: operator_id,
            name: record.operator_name,
        })
    }

    async fn get_station_operator_by_name(db: &PgPool, operator_name: station_operator::Name) -> DatabaseResult<StationOperator> {
        let operator_id = sqlx::query!(
            "SELECT operator_id FROM station_operators WHERE operator_name = $1",
            &operator_name,
        )
            .fetch_one(db)
            .await?
            .operator_id;

        Ok(StationOperator {
            id: operator_id,
            name: operator_name,
        })
    }
}

#[sqlx::test]
async fn test_station_operator(db: PgPool) -> DatabaseResult<()> {
    Postgres::initialize(&db).await?;

    let operator_name1 = "station_operator1".to_string();
    let operator_name2 = "station_operator2".to_string();

    let operator_id1 = Postgres::add_station_operator(&db, operator_name1.clone()).await?;
    let operator_id2 = Postgres::add_station_operator(&db, operator_name2.clone()).await?;

    let operator1_by_id = Postgres::get_station_operator_by_id(&db, operator_id1).await?;
    let operator2_by_id = Postgres::get_station_operator_by_id(&db, operator_id2).await?;

    let operator1_by_name = Postgres::get_station_operator_by_name(&db, operator_name1.clone()).await?;
    let operator2_by_name = Postgres::get_station_operator_by_name(&db, operator_name2.clone()).await?;

    assert!(operator_id1 != operator_id2);

    assert_eq!(operator_name1, operator1_by_id.name);
    assert_eq!(operator_name1, operator1_by_name.name);

    assert_eq!(operator_name2, operator2_by_id.name);
    assert_eq!(operator_name2, operator2_by_name.name);

    Ok(())
}
