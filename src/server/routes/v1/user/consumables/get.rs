#[cfg(feature = "server")]
use {
    crate::server::database::models::user::UserDB, dioxus::server::axum::Extension,
    django_rs::server::database_strategy::DatabaseStrategyError,
};

use dioxus::prelude::*;
use thiserror::Error;
use {
    crate::dtos::consumable::Consumable,
    dioxus::fullstack::AsStatusCode,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum GetConsumableError {
    #[error("Database error")]
    DatabaseError,

    #[error("Server error: {0}")]
    ServerFn(String),

    #[error("Consumable not found: {0}")]
    NotFound(i64),
}

impl From<ServerFnError> for GetConsumableError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for GetConsumableError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Database error: {}", value.to_string());
        Self::DatabaseError
    }
}

impl AsStatusCode for GetConsumableError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            GetConsumableError::ServerFn(_) | GetConsumableError::DatabaseError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            GetConsumableError::NotFound(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/api/v1/consumables", user: Extension<UserDB>)]
pub async fn get_consumables() -> Result<Vec<Consumable>, GetConsumableError> {
    use crate::server::database::models::consumables::ConsumableDB;

    let consumables = ConsumableDB::get_consumable(None, &user)?;

    Ok(consumables)
}

#[get("/api/v1/consumables/:id", user: Extension<UserDB>)]
pub async fn get_consumable(id: i64) -> Result<Consumable, GetConsumableError> {
    use crate::server::database::models::consumables::ConsumableDB;

    let consumables = ConsumableDB::get_consumable(Some(id), &user)?.pop();

    match consumables {
        Some(value) => Ok(value),
        None => Err(GetConsumableError::NotFound(id)),
    }
}
