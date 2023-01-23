use crate::database::Database;

pub struct Postgres;

impl Database<PgPool> for Postgres {

}
