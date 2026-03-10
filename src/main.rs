use dioxus::prelude::*;

use crate::client::launch_client;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod client;
mod server;

fn main() {
    dioxus::fullstack::set_server_url("http://localhost:8080");

    #[cfg(feature = "server")]
    crate::server::entry::launch_server();

    #[cfg(not(feature = "server"))]
    launch_client();
}
