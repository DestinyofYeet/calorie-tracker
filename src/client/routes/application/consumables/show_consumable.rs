use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    client::components::{Select, SelectValue, TextBox, TextBoxType},
    dtos::consumable::{
        Consumable, NutritionEnergy, NutritionValueType, Nutritions, ValueParseError,
    },
};

#[derive(Error, Debug)]
pub enum ConsumableParseError {
    #[error("Failed to parse energy: {0}")]
    Energy(String),

    #[error("Failed to parse value: {0}")]
    ValueParse(String),
}

impl From<ValueParseError> for ConsumableParseError {
    fn from(value: ValueParseError) -> Self {
        Self::ValueParse(value.to_string())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConsumablesForm {
    pub name: String,
    pub serving_size: String,
    pub serving_size_type: String,
    pub energy: String,
    pub energy_type: String,
    pub fat: String,
    pub fat_type: String,
    pub carbohydrates: String,
    pub carbohydrates_type: String,
    pub salt: String,
    pub salt_type: String,
    pub proteins: String,
    pub proteins_type: String,
}

impl TryFrom<ConsumablesForm> for Consumable {
    type Error = ConsumableParseError;

    fn try_from(value: ConsumablesForm) -> std::result::Result<Self, Self::Error> {
        let energy = {
            let energy = if value.energy.contains(".") {
                value
                    .energy
                    .parse::<f64>()
                    .map_err(|e| ConsumableParseError::Energy(e.to_string()))
            } else {
                value
                    .energy
                    .parse::<i64>()
                    .map(|e| e as f64)
                    .map_err(|e| ConsumableParseError::Energy(e.to_string()))
            }?;

            if value.energy_type == "true" {
                NutritionEnergy::as_kcal(energy)
            } else {
                NutritionEnergy::from_kj(energy)
            }
        };

        let nutritions = Nutritions {
            energy,
            fat: NutritionValueType::try_from((value.fat_type, value.fat))?,
            serving_size: NutritionValueType::try_from((
                value.serving_size_type,
                value.serving_size,
            ))?,
            carbohydrates: NutritionValueType::try_from((
                value.carbohydrates_type,
                value.carbohydrates,
            ))?,
            salt: NutritionValueType::try_from((value.salt_type, value.salt))?,
            proteins: NutritionValueType::try_from((value.proteins_type, value.proteins))?,
            extra_values: Vec::new(),
        };

        let consumable = Consumable {
            id: None,
            name: value.name,
            nutritions,
        };

        Ok(consumable)
    }
}

#[component]
pub fn ShowConsumable(consumable: Option<Consumable>) -> Element {
    #[css_module("/src/client/assets/css/application/consumables/add.css")]
    struct Style;

    let text_box_number_kind = TextBoxType::Number {
        min: 0,
        max: i64::MAX,
        step: 0.01,
    };

    rsx! {

        div { class: Style::form,

            label { "Name" }

            TextBox {
                name: "name",
                placeholder: "Name",
                required: true,
                kind: TextBoxType::Text,
                initial_value: consumable.as_ref().map(|e| e.name.clone())
            }

            p {}

            p {}

            p { "Serving Value" }

            p {}

            label { "Serving size" }

            TextBox {
                name: "serving_size",
                placeholder: "0",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|e| e.nutritions.serving_size.get_value().to_string())
            }

            Select {
                name: "serving_size_type",
                required: true,

                options: {
                    match consumable.as_ref() {
                        None => NutritionValueType::get_options(),
                        Some(value) => {
                            let key = value.nutritions.serving_size.get_key();
                            NutritionValueType::get_options_selected(key)
                        }
                    }

                },
            }

            label { "Energy" }

            TextBox {
                name: "energy",
                placeholder: "Energy",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|value| value.nutritions.energy.kcal.to_string())
            }

            Select {
                name: "energy_type",
                required: true,

                options: vec![("kcal", "true"), ("kJ", "false")],
            }

            label { "Fat" }

            TextBox {
                name: "fat",
                placeholder: "0.0",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|value| value.nutritions.fat.get_value().to_string())
            }

            Select {
                name: "fat_type",
                required: true,

                options: match consumable.as_ref() {
                    Some(value) => {
                        let key = value.nutritions.fat.get_key();
                        NutritionValueType::get_options_selected(key)
                    },
                    None => NutritionValueType::get_options(),
                },
            }

            label { "Carbohydrates" }

            TextBox {
                name: "carbohydrates",
                placeholder: "0.0",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|value| value.nutritions.carbohydrates.get_value().to_string())
            }

            Select {
                name: "carbohydrates_type",
                required: true,
                options: match consumable.as_ref() {
                    Some(value) => {
                        let key = value.nutritions.carbohydrates.get_key();
                        NutritionValueType::get_options_selected(key)
                    },
                    None => NutritionValueType::get_options()
                },
            }

            label { "Salt" }

            TextBox {
                name: "salt",
                placeholder: "0.0",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|value| value.nutritions.salt.get_value().to_string())
            }

            Select {
                name: "salt_type",
                required: true,

                options: match consumable.as_ref() {
                    Some(value) => {
                        let key = value.nutritions.salt.get_key();
                        NutritionValueType::get_options_selected(key)

                    },
                    None => NutritionValueType::get_options()
                },
            }

            label { "Proteins" }

            TextBox {
                name: "proteins",
                placeholder: "0.0",
                required: true,
                kind: text_box_number_kind.clone(),
                initial_value: consumable.as_ref().map(|value| value.nutritions.proteins.get_value().to_string())
            }

            Select {
                name: "proteins_type",
                required: true,

                options: match consumable.as_ref() {
                    Some(value) => {
                        let key = value.nutritions.proteins.get_key();
                        NutritionValueType::get_options_selected(key)
                    },
                    None => NutritionValueType::get_options()
                },
            }
        }
    }
}
