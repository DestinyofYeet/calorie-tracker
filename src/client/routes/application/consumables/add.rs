use crate::client::routes::application::consumables::ConsumablesForm;
use dioxus::prelude::*;

use crate::{
    client::{
        components::{Button, Dialog, Spacer},
        routes::application::consumables::ShowConsumable,
        Routes,
    },
    dtos::consumable::Consumable,
    server::routes::v1::user::consumables::create_consumable,
};

#[component]
pub fn ConsumablesAdd() -> Element {
    #[css_module("/src/client/assets/css/application/consumables/add.css")]
    struct Style;

    let mut dialog_open = use_signal(|| false);
    let mut dialog_text = use_signal(String::new);

    rsx! {
        Dialog { text: dialog_text, open: dialog_open }

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
                    }
                };

                let consumable: Consumable = match values.try_into() {
                    Ok(value) => value,
                    Err(e) => {
                        dialog_text.set(e.to_string());
                        dialog_open.set(true);
                        return;
                    },
                };

                match create_consumable(consumable).await {
                    Ok(_) => {
                        let navigator = use_navigator();
                        navigator.push(Routes::ConsumablesManage {});
                    }
                    Err(e) => {
                        dialog_text.set(format!("Failed to create consumable: {e}"));
                        dialog_open.set(true);
                    }
                }
            },

            h3 { "Add a consumable" }

            ShowConsumable {}

            Spacer { rem: 2 }

            Button { "Create consumable" }
        }
    }
}
