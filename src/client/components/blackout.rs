use dioxus::prelude::*;

#[component]
pub fn Blackout(enabled: Signal<bool>, on_click: Option<Callback<MouseEvent>>) -> Element {
    #[css_module("/src/client/assets/css/components/blackout.css")]
    struct Style;

    rsx! {
        div {
            class: Style::blackout,
            onclick: move |e| {
                if let Some(callback) = on_click {
                    callback.call(e);
                }
            },

            style: if enabled() { "display: block;" },
            style: if !enabled() { "display: none;" },
        }
    }
}
