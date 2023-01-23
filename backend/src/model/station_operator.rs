pub type Id = i32;
pub type Name = String;

#[derive(Debug, Clone)]
pub struct StationOperator {
    pub id: Id,
    pub name: Name,
}
