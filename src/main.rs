mod client;
mod dtos;
mod server;

fn main() {
    dioxus::fullstack::set_server_url("http://localhost:8080");

    #[cfg(feature = "server")]
    crate::server::entry::launch_server();

    #[cfg(not(feature = "server"))]
    crate::client::launch_client();
}
