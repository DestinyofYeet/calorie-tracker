use serde::{Deserialize, Serialize};

use crate::dtos::food::Nutritions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumable {
    pub id: Option<i64>,
    pub name: String,
    pub nutritions: Nutritions,
}
