use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[get("/api/text")]
async fn get_text() -> Result<String> {
    Ok("Hi".to_string())
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
