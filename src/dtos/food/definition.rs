use crate::dtos::food::Nutritions;

#[derive(Debug)]
pub struct Food {
    pub id: Option<i64>,
    pub name: String,
    pub nutritions: Nutritions,
}
