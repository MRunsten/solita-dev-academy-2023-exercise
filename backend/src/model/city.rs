pub type Id = i32;

#[derive(Debug, sqlx::FromRow)]
pub struct City {
    pub id: Id,

    #[sqlx(flatten)]
    pub name: Name,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Name {
    pub finnish: String,
    pub swedish: String,
}
