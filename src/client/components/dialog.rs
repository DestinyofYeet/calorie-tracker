use dioxus::prelude::*;

use crate::client::components::{Blackout, Button};

#[component]
pub fn Dialog(text: Signal<String>, open: Signal<bool>) -> Element {
    #[css_module("/src/client/assets/css/components/dialog.css")]
    struct Style;

    rsx! {
        Blackout { enabled: open }

        dialog {
            class: Style::dialog,

            id: "dialog",
            popover: true,
            open,

            {text()}

            Button {
                on_click: move |_| {
                    open.set(false);
                },

                "Close"
            }
        }
    }
}
