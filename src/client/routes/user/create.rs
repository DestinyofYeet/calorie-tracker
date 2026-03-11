use dioxus::prelude::*;

use crate::client::components::TextBox;

static CSS: Asset = asset!(concat!(env!("CSS_ASSET_DIR"), "/user/create.scss"));

#[component]
pub fn UserCreate() -> Element {
    rsx! {
        document::Stylesheet {
            href: CSS
        }

        TextBox {
            placeholder: "Username"
        }
    }
}
