use std::{env, sync::Mutex};

use diesel::{
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
    Connection, SqliteConnection,
};
use dioxus::fullstack::Lazy;

pub mod models;
pub mod schema;

pub static DATABASE: Lazy<Pool<ConnectionManager<SqliteConnection>>> = Lazy::new(|| async move {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    dioxus::Ok(pool)
});
