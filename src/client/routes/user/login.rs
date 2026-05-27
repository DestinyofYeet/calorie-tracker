use crate::{
    client::{
        components::{Button, Spinner, TextBox},
        Routes,
    },
    server::routes::v1::user::login_user,
};
use dioxus::{
    fullstack::{HeaderMap, HeaderValue},
    prelude::*,
};

#[component]
pub fn UserLogin() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    // dioxus::fullstack::set_request_headers(headers);

    let mut button_disabled = use_signal(|| false);
    let mut error = use_signal(String::new);

    let login_user = move || async move {
        match login_user(email(), password()).await {
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

                    "Login to Calorie-Tracker"

                    div {}

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
                                    match login_user().await {
                                        Some(_) => {
                                            navigator().replace(Routes::Landing {});

                                        },
                                        None => {
                                            button_disabled.set(false);
                                        },
                                    }
                                });
                            },

                            "Login"
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
                                "Logging in"
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
