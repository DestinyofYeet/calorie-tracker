use serde::{Deserialize, Serialize};

use crate::dtos::consumable::Nutritions;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Consumable {
    pub id: Option<i64>,
    pub name: String,
    pub nutritions: Nutritions,
}
