use dioxus::prelude::*;

use crate::{
    client::Routes,
    server::{get_text, routes::v1::user::is_authed::is_user_authed},
};

#[component]
pub fn Landing() -> Element {
    // let text = use_resource(move || async move { get_text().await });
    let is_authed = use_resource(move || async move { is_user_authed().await });

    let nav = navigator();

    rsx! {
        match is_authed.read().as_ref() {
            Some(Ok(value)) => match value {
                true => {
                    nav.replace(Routes::ApplicationStart {});
                    "Authed."
                },
                false => {
                    nav.replace(Routes::UserLogin {  } );
                    "You are being redirected for login"
                },
            },
            _ => "Checking for auth...",
        }
    }
}
