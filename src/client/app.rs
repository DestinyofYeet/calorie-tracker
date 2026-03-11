use crate::client::routes::user::UserCreate;
use dioxus::prelude::*;

use crate::client::routes::Landing;

const VARS_CSS: Asset = asset!("/src/client/assets/css/variables.css");
const BASE_CSS: Asset = asset!("/src/client/assets/css/base.css");

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
        document::Stylesheet { href: VARS_CSS }
        document::Stylesheet { href: BASE_CSS }
        Router::<Route> {}
    }
}
