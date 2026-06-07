use crate::client::Routes;
use crate::dtos::consumable::Consumable;
use crate::{
    client::routes::application::consumables::ConsumablesForm,
    server::routes::v1::user::consumables::update_consumable,
};
use dioxus::prelude::*;

use crate::{
    client::{
        components::{Button, Dialog, Spacer},
        routes::application::consumables::ShowConsumable,
    },
    server::routes::v1::user::consumables::get_consumable,
};

#[component]
pub fn ConsumableModify(consumable_id: i64) -> Element {
    #[css_module("/src/client/assets/css/application/consumables/add.css")]
    struct Style;

    let mut dialog_text = use_signal(String::new);
    let mut dialog_open = use_signal(|| false);

    let consumable = use_resource(move || async move { get_consumable(consumable_id).await });
    rsx! {
        Dialog { text: dialog_text, open: dialog_open }

        match consumable.read().as_ref() {
            None => rsx! { "Loading consumable..." },
            Some(Err(e)) => {
                dialog_text.set(format!("Failed to load consumable: {}", e));
                dialog_open.set(true);
                rsx! {}

            }
            Some(Ok(value)) => {
                let consumable_id = value.id;
                rsx! {
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

                            let mut consumable: Consumable = match values.try_into() {
                                Ok(value) => value,
                                Err(e) => {
                                    dialog_text.set(format!("Failed to parse consumable:\n{e}"));
                                    dialog_open.set(true);
                                    return;
                                }
                            };

                            consumable.id = consumable_id;

                            match update_consumable(consumable).await {
                                Ok(_) => {
                                    let navigator = use_navigator();
                                    navigator.push(Routes::ConsumablesManage {});
                                }
                                Err(e) => {
                                    dialog_text.set(format!("Failed to update consumable: {e}"));
                                    dialog_open.set(true);
                                }
                            }
                        },

                        h3 { "Modify a consumable" }

                        ShowConsumable { consumable: value.clone() }

                        Spacer { rem: 2 }

                        Button { "Save" }
                    }
                }
            }
        }
    }
}
