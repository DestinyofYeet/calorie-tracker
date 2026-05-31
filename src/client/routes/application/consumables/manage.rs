use dioxus::prelude::*;

use crate::client::Routes;

#[component]
pub fn ConsumablesManage() -> Element {
    rsx! {
        p { "Manage consumables" }

        Link { to: Routes::ConsumablesAdd {}, "Add consumable" }
    }
}
