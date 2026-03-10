use dioxus::prelude::*;

use crate::server::get_text;

pub fn launch_client() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    let text = use_resource(move || async move { get_text().await });
    rsx! {
        if let None = *text.read() {
            "Loading..."
        } else {
            match &text.read().as_ref().unwrap() {
                Ok(e) => e.to_string(),
                Err(e) => format!("Failed to fetch text: {e}"),
            }
        }
    }
}
