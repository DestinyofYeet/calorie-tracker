#![cfg(feature = "server")]
use std::env;

use dioxus::{fullstack::Lazy, server::axum};
use django_rs::{
    server::{
        database_strategy::{default_strategies::SqliteStrategy, DatabaseStrategy},
        Server,
    },
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};
use tracing_subscriber::EnvFilter;

use crate::{
    client::App,
    server::{
        database::models::{login_token::LoginToken, user::User},
        layers::authenticated::run_authenticated_layer,
    },
};

pub static SERVER: Lazy<Server<SqliteStrategy>> = Lazy::new(async || {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let server = Server::new(8, TracingStrategy {}, SqliteStrategy::new(database_url))
        .expect("To create server");

    dioxus::Ok(server)
});

pub fn launch_server() {
    let level = "trace,dioxus_server=warn,tungstenite=warn,dioxus_core=warn,warnings=warn,dioxus_signals=warn,django_rs=info";

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    let db = SERVER.get_database();

    db.migrate_model::<User>().unwrap();
    db.migrate_model::<LoginToken>().unwrap();

    dioxus::serve(|| async move {
        Ok(dioxus::server::router(App).layer(axum::middleware::from_fn(run_authenticated_layer)))
    })
}
