use dioxus::prelude::*;

use crate::client::components::TextBox;

#[component]
pub fn UserCreate() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    rsx! {
        div {

            class: Style::wrapper_wrapper,

            div {
                class: Style::login_wrapper,

                div {}

                div {
                    class: Style::login,

                    TextBox {
                        placeholder: "Username"
                    }

                    TextBox {
                        placeholder: "Password",
                        kind: "Password"
                    }
                }

            }
        }

    }
}
