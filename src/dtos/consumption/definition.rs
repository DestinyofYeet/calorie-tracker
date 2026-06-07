use crate::dtos::{consumable::NutritionValue, consumption::ConsumptionType};

pub struct Consumption {
    pub id: Option<i64>,
    pub user_id: i64,
    pub nutritions: NutritionValue,
    pub kind: ConsumptionType,
}
