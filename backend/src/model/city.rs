pub type Id = i32;

#[derive(Debug)]
pub struct City {
    pub id: Id,
    pub name: Name,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub finnish: String,
    pub swedish: String,
}
