pub type Id = i32;

#[derive(Debug)]
pub struct City {
    pub id: Id,
    pub name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name {
    pub finnish: String,
    pub swedish: String,
}
