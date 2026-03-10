#![cfg(feature = "server")]
use dioxus::server::axum::{self};
use tracing_subscriber::EnvFilter;

use crate::{server::layers::authenticated::run_authenticated_layer, App};

pub fn launch_server() {
    let level = "trace,dioxus_server=warn,tungstenite=warn,dioxus_core=warn,warnings=warn,dioxus_signals=warn";
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();
    dioxus::serve(|| async move {
        Ok(dioxus::server::router(App).layer(axum::middleware::from_fn(run_authenticated_layer)))
    })
}
