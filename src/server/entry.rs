#![cfg(feature = "server")]
use std::process::exit;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dioxus::server::axum::{self};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::{
    client::App,
    server::{database::DATABASE, layers::authenticated::run_authenticated_layer},
};

pub fn launch_server() {
    let level = "trace,dioxus_server=warn,tungstenite=warn,dioxus_core=warn,warnings=warn,dioxus_signals=warn";

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    let mut db = match DATABASE.get().get() {
        Err(e) => {
            error!("Failed to get database: {e}");
            exit(1);
        }
        Ok(value) => value,
    };

    match db.run_pending_migrations(MIGRATIONS) {
        Err(e) => {
            error!("Failed to run migrations: {e}");
            exit(1)
        }
        Ok(value) => {
            info!("Successfully applied {} migrations", value.len())
        }
    }

    dioxus::serve(|| async move {
        Ok(dioxus::server::router(App).layer(axum::middleware::from_fn(run_authenticated_layer)))
    })
}
