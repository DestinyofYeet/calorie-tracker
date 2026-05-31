use crate::server::dtos::food::Nutritions;

pub struct Food {
    id: i64,
    name: String,
    nutritions: Nutritions,
}
