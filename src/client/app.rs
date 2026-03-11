use crate::client::routes::user::UserCreate;
use dioxus::prelude::*;

use crate::client::routes::Landing;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Landing {},

    #[route("/user/create")]
    UserCreate {},
}

pub fn launch_client() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
