use dioxus::{core::anyhow, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    client::components::{Button, Dialog, Select, Spacer, TextBox, TextBoxType},
    dtos::food::{Consumable, NutritionEnergy, NutritionValueType, Nutritions, ValueParseError},
    server::routes::v1::user::consumables::create_consumable,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConsumablesForm {
    name: String,
    serving_size: String,
    serving_size_type: String,
    energy: String,
    energy_type: String,
    fat: String,
    fat_type: String,
    carbohydrates: String,
    carbohydrates_type: String,
    salt: String,
    salt_type: String,
    proteins: String,
    proteins_type: String,
}

#[component]
pub fn ConsumablesAdd() -> Element {
    #[css_module("/src/client/assets/css/application/consumables/add.css")]
    struct Style;

    let mut dialog_open = use_signal(|| false);
    let mut dialog_text = use_signal(String::new);

    let text_box_number_kind = TextBoxType::Number {
        min: 0,
        max: i64::MAX,
        step: 0.01,
    };

    rsx! {
        Dialog {
            text: dialog_text,
            open: dialog_open,
        }

        form {
            class: Style::formwrapper,

            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();

                let values: ConsumablesForm = match evt.parsed_values() {
                    Ok(value) => value,
                    Err(e) => {
                        dialog_text.set(format!("Failed to parse form:\n{e}"));
                        dialog_open.set(true);
                        return;
                    },
                };

                let energy = {
                    let energy = if values.energy.contains(".") {
                        values.energy.parse::<f64>().map_err(|e| anyhow!("Failed to parse energy: {e}"))
                    } else {
                        values.energy.parse::<i64>().map(|e| e as f64).map_err(|e| anyhow!("Failed to parse energy: {e}"))
                    };

                    let energy = match energy {
                        Ok(value) => value,
                        Err(e) => {
                            dialog_text.set(format!("Failed to parse enery: {e}"));
                            dialog_open.set(true);
                            return;
                        },
                    };

                    if values.energy_type == "true" {
                        NutritionEnergy::as_kcal(energy)
                    } else {
                        NutritionEnergy::from_kj(energy)
                    }

                };

                let nutritions = move || -> Result<Nutritions, ValueParseError>{
                  Ok(Nutritions { energy, fat: NutritionValueType::try_from((values.fat_type, values.fat))?,
                       serving_size: NutritionValueType::try_from((values.serving_size_type, values.serving_size))?,
                       carbohydrates: NutritionValueType::try_from((values.carbohydrates_type, values.carbohydrates))?,
                       salt: NutritionValueType::try_from((values.salt_type, values.salt))?,
                       proteins: NutritionValueType::try_from((values.proteins_type, values.proteins))?,
                       extra_values: Vec::new() })
                }();

                let nutritions = match nutritions {
                    Ok(values) => values,
                    Err(e) => {
                        dialog_text.set(format!("Failed to parse nutritions: {e}"));
                        dialog_open.set(true);
                        return;
                    },
                };

                let consumable = Consumable {
                    id: None,
                    name: values.name,
                    nutritions
                };

                match create_consumable(consumable).await {
                    Ok(_) => {},
                    Err(e) => {
                        dialog_text.set(format!("Failed to save consumable: {e}"));
                        dialog_open.set(true);
                    },
                }

            },

            h3 { "Add a consumable" }

            div { class: Style::form,

                label { "Name" }

                TextBox { name: "name", placeholder: "Name", required: true, kind: TextBoxType::Text}

                p {}

                p {}

                p {
                    "Serving Values"
                }

                p {}

                label { "Serving size" }

                TextBox { name: "serving_size", placeholder: "0", required: true, kind: text_box_number_kind.clone()}

                Select {
                    name: "serving_size_type",
                    required: true,

                    options: NutritionValueType::get_options(),
                }

                label { "Energy" }

                TextBox { name: "energy", placeholder: "Energy", required: true, kind: text_box_number_kind.clone()}

                Select {
                    name: "energy_type",
                    required: true,

                    options: vec![("kcal", "true"), ("kJ", "false")],
                }

                label { "Fat" }

                TextBox { name: "fat", placeholder: "0.0", required: true, kind: text_box_number_kind.clone(),}

                Select {
                    name: "fat_type",
                    required: true,

                    options: NutritionValueType::get_options(),
                }

                label { "Carbohydrates" }

                TextBox { name: "carbohydrates", placeholder: "0.0", required: true, kind: text_box_number_kind.clone()}

                Select {
                    name: "carbohydrates_type",
                    required: true,
                    options: NutritionValueType::get_options(),
                }

                label { "Salt" }

                TextBox { name: "salt", placeholder: "0.0", required: true, kind: text_box_number_kind.clone()}

                Select {
                    name: "salt_type",
                    required: true,

                    options: NutritionValueType::get_options(),
                }

                label { "Proteins" }

                TextBox { name: "proteins", placeholder: "0.0", required: true, kind: text_box_number_kind.clone()}

                Select {
                    name: "proteins_type",
                    required: true,

                    options: NutritionValueType::get_options(),
                }
            }

            Spacer { rem: 2 }

            Button { "Create consumable" }
        }
    }
}
