use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdPlus, LdSearch},
    Icon,
};
use tracing::Instrument;

use crate::{
    client::{
        components::{Button, TextBox, TextBoxType},
        Routes,
    },
    dtos::food::Consumable,
    server::routes::v1::user::consumables::get_consumables,
};

#[component]
pub fn ConsumablesManage() -> Element {
    #[css_module("/src/client/assets/css/application/consumables/manage.css")]
    struct Style;

    let consumables = use_resource(async move || get_consumables().await);

    let mut search_input = use_signal(String::new);
    rsx! {
        div {
            class: Style::header,

            "Manage consumables"

            TextBox { placeholder: "Search", kind: TextBoxType::Text,
                on_input: move |e: Event<FormData>| {
                    search_input.set(e.value());
                }
            }

            Link { to: Routes::ConsumablesAdd {}, Button {
                div {
                    class: Style::centerbuttoncontent,

                    "Add"
                    Icon {
                        icon: LdPlus {}
                    }
                }
            }}
        }

        match &*consumables.read() {
            None => rsx!{ p { "Loading consumables" } },
            Some(Err(e)) => rsx!{
                p { "Failed to load consumables: " {e.to_string()} }
            },
            Some(Ok(value)) => {
                rsx! {
                    div {
                        class: Style::items,

                        p {
                            class: Style::gridheaderitem,
                            "Name"
                        }
                        p {
                            class: Style::gridheaderitem,
                            "Data"
                        }

                        for consumable in value.iter().filter(|element| {
                            if search_input().is_empty() { true } else {
                                element.name.starts_with(&search_input())
                            }
                        }) {
                            RenderConsumableRow { consumable: consumable.clone()  }
                        }
                    }

                }
            }
        }


    }
}

#[component]
pub fn RenderConsumableRow(consumable: Consumable) -> Element {
    #[css_module("/src/client/assets/css/application/consumables/render_consumable_row.css")]
    struct Style;

    rsx! {
        p { class: Style::title,
            {consumable.name}
        }
        div {
            class: Style::data,
            p { {consumable.nutritions.energy.to_string()} }
            p { {consumable.nutritions.fat.to_string() + " fat"} }
        }

    }
}
