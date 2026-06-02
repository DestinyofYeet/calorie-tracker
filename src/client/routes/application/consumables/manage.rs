use dioxus::prelude::*;

use crate::{client::Routes, server::routes::v1::user::consumables::get_consumables};

#[component]
pub fn ConsumablesManage() -> Element {
    let consumables = use_resource(async move || get_consumables().await);
    rsx! {
        Link { to: Routes::ConsumablesAdd {}, "Add consumable" }

        match &*consumables.read() {
            None => rsx!{ p { "Loading consumables" } },
            Some(Err(e)) => rsx!{
                p { "Failed to load consumables: " {e.to_string()} }
            },
            Some(Ok(value)) => {
                rsx! {
                    for consumable in value.iter() {
                        p { "Name: " {consumable.name.clone()} }
                    }

                }
            }
        }


    }
}
