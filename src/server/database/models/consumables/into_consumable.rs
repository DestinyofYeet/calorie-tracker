use crate::{
    dtos::consumable::{Consumable, Nutritions},
    server::database::models::consumables::ConsumableDB,
};

impl From<ConsumableDB> for Consumable {
    fn from(val: ConsumableDB) -> Self {
        let nutritions = Nutritions {
            energy: val.energy,
            fat: val.fat,
            serving_size: val.serving_size,
            carbohydrates: val.carbohydrates,
            salt: val.salt,
            proteins: val.proteins,
            extra_values: val.extra_values,
        };
        Consumable {
            id: val.id,
            name: val.name,
            nutritions,
        }
    }
}
