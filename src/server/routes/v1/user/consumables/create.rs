use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dtos::food::Consumable;

#[cfg(feature = "server")]
use {
    crate::server::database::models::consumables::ConsumableDB,
    crate::server::database::models::user::UserDB, dioxus::server::axum::Extension,
    django_rs::server::database_strategy::DatabaseStrategyError,
};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateConsumableError {
    #[error("Server error: {0}")]
    ServerFn(String),

    #[error("Database error")]
    Database,
}

impl From<ServerFnError> for CreateConsumableError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for CreateConsumableError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Database error: {}", value.to_string());
        Self::Database
    }
}

impl AsStatusCode for CreateConsumableError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            CreateConsumableError::Database | CreateConsumableError::ServerFn(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[post("/api/v1/consumables", user: Extension<UserDB>)]
pub async fn create_consumable(mut data: Consumable) -> Result<(), CreateConsumableError> {
    // discard user provided id
    data.id = None;

    ConsumableDB::save_consumable(data, &user)?;

    Ok(())
}
