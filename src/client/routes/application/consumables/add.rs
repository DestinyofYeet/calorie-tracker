use dioxus::prelude::*;

use crate::client::components::TextBox;

#[component]
pub fn ConsumablesAdd() -> Element {
    #[css_module("/src/client/assets/css/application/consumables/add.css")]
    struct Style;

    rsx! {
        div { class: Style::formwrapper,

            form { class: Style::form,

                label { "Name" }

                TextBox { id: "name", placeholder: "name" }
            }
        }
    }
}
