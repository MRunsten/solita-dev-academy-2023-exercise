use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

use crate::database::{Database, DatabaseResult};
use crate::model::{city, station_operator};
use crate::model::city::City;
use crate::model::station_operator::StationOperator;

pub struct Postgres;

#[async_trait]
impl Database<PgPool> for Postgres {
    async fn connect() -> DatabaseResult<PgPool> {
        let postgres_address =
            env::var("DATABASE_URL").expect("Environment variable DATABASE_URL was undefined");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*postgres_address)
            .await?;

        Postgres::initialize(&pool).await?;

        Ok(pool)
    }

    async fn initialize(db: &PgPool) -> DatabaseResult<()> {
        let _ = sqlx::query_file!("queries/postgres/create_table_cities.sql")
            .execute(db)
            .await?;

        let _ = sqlx::query_file!("queries/postgres/create_table_station_operators.sql")
            .execute(db)
            .await?;

        Ok(())
    }

    async fn add_city(db: &PgPool, name: city::Name) -> DatabaseResult<city::Id> {
        let city_id = sqlx::query!(
            "INSERT INTO cities (name_finnish, name_swedish) VALUES ($1, $2) RETURNING city_id",
            &name.finnish,
            &name.swedish
        )
        .fetch_one(db)
        .await?
        .city_id;

        Ok(city_id)
    }

    async fn get_city_by_id(db: &PgPool, city_id: city::Id) -> DatabaseResult<City> {
        let record = sqlx::query!(
            "SELECT name_finnish, name_swedish FROM cities WHERE city_id = $1",
            &city_id
        )
        .fetch_one(db)
        .await?;

        Ok(City {
            id: city_id,
            name: city::Name {
                finnish: record.name_finnish,
                swedish: record.name_swedish,
            },
        })
    }

    async fn get_city_by_name(db: &PgPool, city_name: city::Name) -> DatabaseResult<City> {
        let city_id = sqlx::query!(
            "SELECT city_id FROM cities WHERE name_finnish = $1 and name_swedish = $2",
            &city_name.finnish,
            &city_name.swedish,
        )
        .fetch_one(db)
        .await?
        .city_id;

        Ok(City {
            id: city_id,
            name: city_name,
        })
    }

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
async fn postgres_test_city(db: PgPool) -> DatabaseResult<()> {
    Postgres::initialize(&db).await?;

    let city_name1 = city::Name {
        finnish: "finnish_name1".to_string(),
        swedish: "swedish_name1".to_string(),
    };

    let city_name2 = city::Name {
        finnish: "finnish_name2".to_string(),
        swedish: "swedish_name2".to_string(),
    };

    let city_id1 = Postgres::add_city(&db, city_name1.clone()).await?;
    let city_id2 = Postgres::add_city(&db, city_name2.clone()).await?;

    let city1_by_id = Postgres::get_city_by_id(&db, city_id1).await?;
    let city2_by_id = Postgres::get_city_by_id(&db, city_id2).await?;

    let city1_by_name = Postgres::get_city_by_name(&db, city_name1.clone()).await?;
    let city2_by_name = Postgres::get_city_by_name(&db, city_name2.clone()).await?;

    assert!(city_id1 != city_id2);

    assert_eq!(city_name1.finnish, city1_by_id.name.finnish);
    assert_eq!(city_name1.swedish, city1_by_id.name.swedish);

    assert_eq!(city_name1.finnish, city1_by_name.name.finnish);
    assert_eq!(city_name1.swedish, city1_by_name.name.swedish);

    assert_eq!(city_name2.finnish, city2_by_id.name.finnish);
    assert_eq!(city_name2.swedish, city2_by_id.name.swedish);

    assert_eq!(city_name2.finnish, city2_by_name.name.finnish);
    assert_eq!(city_name2.swedish, city2_by_name.name.swedish);

    Ok(())
}

#[sqlx::test]
async fn postgres_test_station_operator(db: PgPool) -> DatabaseResult<()> {
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

    assert_eq!(operator_name1, operator1_by_id.name);
    assert_eq!(operator_name2, operator2_by_name.name);

    Ok(())
}

