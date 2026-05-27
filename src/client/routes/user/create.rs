use dioxus::prelude::*;

use crate::{
    client::{
        components::{Button, Spinner, TextBox},
        Routes,
    },
    server::routes::v1::user::create_user,
};

#[component]
pub fn UserCreate() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    let mut button_disabled = use_signal(|| false);

    let mut error = use_signal(String::new);

    let create_user = move || async move {
        match create_user(username(), email(), password()).await {
            Ok(_) => Some(()),
            Err(e) => {
                error.set(e.to_string());
                None
            }
        }
    };

    rsx! {
        div {

            class: Style::wrapper_wrapper,

            div {
                class: Style::create_wrapper,

                div {}

                div {
                    class: Style::create,

                    "Username"

                    TextBox {
                        placeholder: "Username",
                        on_input: move |e: Event<FormData>| {username.set(e.value())}
                    }

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


                    if !button_disabled() {
                        Button {
                            disabled: button_disabled,
                            on_click: move |_| {
                                button_disabled.set(true);
                                spawn(async move {
                                    match create_user().await {
                                        None => {
                                            button_disabled.set(false)
                                        },
                                        Some(_) => {
                                            let navigator = navigator();
                                            navigator.replace(Routes::UserLogin {});

                                        }
                                    }
                                });
                            },

                            "Register"
                        }

                        if !error().is_empty() {
                            p {
                                class: "error",

                                {error()}
                            }
                        }
                    } else {
                        div {
                            class: Style::spinner_wrapper,
                            p {
                                style: "margin: 0;",
                                "Creating user"
                            }
                            div {
                                class: Style::spinner,

                                Spinner {}
                            }
                        }
                    }
                }

            }
        }

    }
}
