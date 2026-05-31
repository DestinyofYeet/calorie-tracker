use crate::client::components::SiteOverlay;
use crate::client::routes::application::consumables::{ConsumablesAdd, ConsumablesManage};
use crate::client::routes::application::consumption::ConsumptionAdd;
use crate::client::routes::application::start::ApplicationStart;
use crate::client::routes::user::UserCreate;
use crate::client::routes::user::UserLogin;
use dioxus::prelude::*;

use crate::client::routes::Landing;

const VARS_CSS: Asset = asset!("/src/client/assets/css/variables.css");
const BASE_CSS: Asset = asset!("/src/client/assets/css/base.css");

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Routes {
    #[route("/")]
    Landing {},

    #[route("/user/create")]
    UserCreate {},

    #[route("/user/login")]
    UserLogin {},

    #[layout(SiteOverlayWrapper)]
    #[nest("/application")]
        #[route("/consumption/add")]
        ConsumptionAdd {},

        #[nest("/consumables")]
            #[route("/consumables/add")]
            ConsumablesAdd {},
        #[end_nest]
        #[route("/consumables")]
        ConsumablesManage {},
    #[end_nest]
    #[route("/application")]
    ApplicationStart {},


}

// this is a clippy lie, the code does actually get used
// but depending on the lsp config, the rust-analyzer only evaluates
// the server and thinks this code is dead
#[allow(dead_code)]
pub fn launch_client() {
    dioxus::launch(App);
}

#[component]
pub fn SiteOverlayWrapper() -> Element {
    rsx! {
        SiteOverlay { Outlet::<Routes> {} }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: VARS_CSS }
        document::Stylesheet { href: BASE_CSS }

        Router::<Routes> {}

    }
}
