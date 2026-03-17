use crate::client::components::{Button, TextBox};
use dioxus::prelude::*;

#[component]
pub fn UserLogin() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        div {
            class: Style::wrapper_wrapper,

            div {
                class: Style::create_wrapper,

                div {}

                div {
                    class: Style::create,

                    "Email"

                    TextBox {
                        placeholder: "Email",
                        kind: "Email",
                        on_input: move |e: Event<FormData>| {email.set(e.value())}
                    }

                    "Password"

                    TextBox {
                        placeholder: "Password",
                        kind: "Password",
                        on_input: move |e: Event<FormData>| {password.set(e.value())}
                    }

                    Button {
                        text: "Login"
                    }
                }
            }
        }
    }
}
