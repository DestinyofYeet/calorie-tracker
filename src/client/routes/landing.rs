use dioxus::prelude::*;

use crate::{client::Routes, server::get_text};

#[component]
pub fn Landing() -> Element {
    let text = use_resource(move || async move { get_text().await });

    rsx! {
        if let Some(value) = text.read().as_ref() {
            match value {
                Ok(value) => {
                    value.to_string()
                },
                Err(e) => {
                        e.to_string()
                },
            }
        } else {
                "Loading ..."
        }

        Link {
            to: Routes::UserLogin {  },
            "Login"
        }
    }
}
