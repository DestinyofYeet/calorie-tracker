use dioxus::prelude::*;

#[component]
pub fn UserLogin() -> Element {
    #[css_module("/src/client/assets/css/user/create.css")]
    struct Style;

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        p {
            "hi"
        }
    }
}
