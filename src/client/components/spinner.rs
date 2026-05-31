use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    #[css_module("/src/client/assets/css/components/spinner.css")]
    struct Style;

    rsx! {
        div { class: Style::spinner }
    }
}
