use crate::{
    client::{
        components::{Button, Spinner, TextBox, TextBoxType},
        Routes,
    },
    dtos::LoginUser,
    server::routes::v1::user::login_user,
};
use dioxus::prelude::*;

#[component]
pub fn UserLogin() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    // dioxus::fullstack::set_request_headers(headers);

    let mut button_disabled = use_signal(|| false);
    let mut error = use_signal(String::new);

    let mut login_user = move || {
        button_disabled.set(true);
        spawn(async move {
            let nav = navigator();
            let login_user_obj = LoginUser {
                email: email(),
                password: password(),
            };

            match login_user(login_user_obj).await {
                Ok(_) => {
                    nav.replace(Routes::Landing {});
                }
                Err(e) => {
                    error.set(e.to_string());
                    button_disabled.set(false);
                }
            };
        });
    };

    rsx! {
        div { class: Style::wrapper_wrapper,

            div { class: Style::create_wrapper,

                div {}

                form {
                    class: Style::create,

                    onsubmit: move |evt| {
                        evt.prevent_default();
                        login_user()
                    },

                    "Login to Calorie-Tracker"

                    div {}

                    "Email"

                    TextBox {
                        placeholder: "Email",
                        kind: TextBoxType::Email,
                        on_input: move |e: Event<FormData>| { email.set(e.value()) },
                    }

                    "Password"

                    TextBox {
                        placeholder: "Password",
                        kind: TextBoxType::Password,
                        on_input: move |e: Event<FormData>| { password.set(e.value()) },
                    }

                    if !button_disabled() {
                        Button {
                            disabled: button_disabled,
                            on_click: move |_| {
                                login_user();
                            },

                            "Login"
                        }

                        if !error().is_empty() {
                            p { class: "error", {error()} }
                        }
                    } else {
                        div { class: Style::spinner_wrapper,
                            p { style: "margin: 0;", "Logging in" }
                            div { class: Style::spinner, Spinner {} }
                        }
                    }

                }
            }
        }
    }
}
