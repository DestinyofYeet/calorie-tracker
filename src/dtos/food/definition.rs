use crate::dtos::food::Nutritions;

pub struct Food {
    id: Option<i64>,
    name: String,
    nutritions: Nutritions,
}
