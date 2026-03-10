use dioxus::prelude::*;

use crate::server::get_text;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod server;

fn main() {
    dioxus::fullstack::set_server_url("http://localhost:8080");

    #[cfg(feature = "server")]
    crate::server::entry::launch_server();

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
