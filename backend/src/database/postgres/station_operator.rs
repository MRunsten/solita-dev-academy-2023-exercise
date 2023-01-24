use sqlx::PgPool;

use crate::database::DatabaseResult;
use crate::model::station_operator;
use crate::model::station_operator::StationOperator;

pub async fn add(db: &PgPool, name: station_operator::Name) -> DatabaseResult<station_operator::Id> {
    let operator_id = sqlx::query!(
            "INSERT INTO station_operators (operator_name) VALUES ($1) RETURNING operator_id",
            &name,
        )
        .fetch_one(db)
        .await?
        .operator_id;

    Ok(operator_id)
}

pub async fn get_by_id(db: &PgPool, operator_id: station_operator::Id) -> DatabaseResult<StationOperator> {
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

pub async fn get_by_name(db: &PgPool, operator_name: station_operator::Name) -> DatabaseResult<StationOperator> {
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

#[sqlx::test]
async fn test_station_operator(db: PgPool) -> DatabaseResult<()> {
    crate::database::initialize(&db).await?;

    let operator_name1 = "station_operator1".to_string();
    let operator_name2 = "station_operator2".to_string();

    let operator_id1 = add(&db, operator_name1.clone()).await?;
    let operator_id2 = add(&db, operator_name2.clone()).await?;

    let operator1_by_id = get_by_id(&db, operator_id1).await?;
    let operator2_by_id = get_by_id(&db, operator_id2).await?;

    let operator1_by_name = get_by_name(&db, operator_name1.clone()).await?;
    let operator2_by_name = get_by_name(&db, operator_name2.clone()).await?;

    assert!(operator_id1 != operator_id2);

    assert_eq!(operator_name1, operator1_by_id.name);
    assert_eq!(operator_name1, operator1_by_name.name);

    assert_eq!(operator_name2, operator2_by_id.name);
    assert_eq!(operator_name2, operator2_by_name.name);

    crate::database::empty(&db).await?;

    let operator1_by_id_result = get_by_id(&db, operator_id1).await;
    let operator2_by_id_result = get_by_id(&db, operator_id2).await;

    assert!(operator1_by_id_result.is_err());
    assert!(operator2_by_id_result.is_err());

    Ok(())
}
