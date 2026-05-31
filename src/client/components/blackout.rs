use dioxus::prelude::*;

#[component]
pub fn Blackout(enabled: Signal<bool>) -> Element {
    #[css_module("/src/client/assets/css/components/blackout.css")]
    struct Style;

    rsx! {
        div {
            class: Style::blackout,

            style: if enabled() { "display: block;" },
            style: if !enabled() { "display: none;" },
        }
    }
}
