use dioxus::prelude::*;

#[component]
pub fn Spacer(rem: i64) -> Element {
    rsx! {
        div { style: format!("height:{}rem;", rem) }
    }
}
