#![cfg(feature = "server")]
use std::process::exit;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dioxus::server::axum::{self};
use tracing::error;
use tracing_subscriber::EnvFilter;

use crate::{
    server::{database::DATABASE, layers::authenticated::run_authenticated_layer},
    App,
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

    if let Err(e) = db.run_pending_migrations(MIGRATIONS) {
        error!("Failed to run migrations: {e}");
        exit(1)
    }

    dioxus::serve(|| async move {
        Ok(dioxus::server::router(App).layer(axum::middleware::from_fn(run_authenticated_layer)))
    })
}
