use dioxus::prelude::*;

#[component]
pub fn TextBox(placeholder: String, kind: Option<String>) -> Element {
    #[css_module("/src/client/assets/css/components/text_box.css")]
    struct Style;

    rsx! {
        input {
            class: Style::input,

            placeholder: placeholder,
            type: if kind.is_some() { kind.unwrap() }
        }
    }
}
