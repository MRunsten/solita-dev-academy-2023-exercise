use async_trait::async_trait;
use sqlx::PgPool;
use crate::database::{CityDatabase, DatabaseResult};
use crate::model::city;
use crate::model::city::City;
use crate::postgres::Postgres;
use crate::database::Database;

#[async_trait]
impl CityDatabase<PgPool> for Postgres {
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

    Postgres::empty(&db).await?;

    let city1_by_id_result = Postgres::get_city_by_id(&db, city_id1).await;
    let city2_by_id_result = Postgres::get_city_by_id(&db, city_id2).await;

    assert!(city1_by_id_result.is_err());
    assert!(city2_by_id_result.is_err());

    Ok(())
}
