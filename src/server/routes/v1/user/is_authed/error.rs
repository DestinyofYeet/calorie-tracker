use dioxus::{fullstack::AsStatusCode, prelude::*};
#[cfg(feature = "server")]
use django_rs::server::database_strategy::DatabaseStrategyError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum IsAuthedError {
    #[error("Server error: {0}")]
    ServerFn(String),

    #[error("Database error")]
    Database,
}

impl From<ServerFnError> for IsAuthedError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

impl AsStatusCode for IsAuthedError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            IsAuthedError::Database | IsAuthedError::ServerFn(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for IsAuthedError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Database error: {}", value.to_string());
        Self::Database
    }
}
