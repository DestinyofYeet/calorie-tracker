use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css(css, "src/client/assets/css/components/text_box.scss")]
pub fn TextBox(placeholder: String) -> Element {
    rsx! {
        input {
            class: Styles::input,

            placeholder: placeholder
        }
    }
}
