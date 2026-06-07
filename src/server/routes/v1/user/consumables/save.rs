use crate::dtos::consumable::Consumable;
use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[cfg(feature = "server")]
use {
    crate::server::database::models::consumables::ConsumableDB,
    crate::server::database::models::user::UserDB, dioxus::server::axum::Extension,
    django_rs::server::database_strategy::DatabaseStrategyError,
};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum UpdateConsumableError {
    #[error("Server error: {0}")]
    ServerFn(String),

    #[error("Database error")]
    DatabaseError,
}

impl From<ServerFnError> for UpdateConsumableError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for UpdateConsumableError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Database error: {}", value.to_string());
        Self::DatabaseError
    }
}

impl AsStatusCode for UpdateConsumableError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            UpdateConsumableError::DatabaseError | UpdateConsumableError::ServerFn(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[put("/api/v1/consumables", user: Extension<UserDB>)]
pub async fn update_consumable(data: Consumable) -> Result<(), UpdateConsumableError> {
    // this function also checks if the consumable belongs to the user
    ConsumableDB::save_consumable(data, &user)?;
    Ok(())
}
