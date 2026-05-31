use dioxus::prelude::*;

use crate::client::components::SiteOverlay;

#[component]
pub fn ConsumptionAdd() -> Element {
    #[css_module("/src/client/assets/css/application/consumption/add.css")]
    struct Style;

    rsx! {
        p { "Add consumption" }
    }
}
