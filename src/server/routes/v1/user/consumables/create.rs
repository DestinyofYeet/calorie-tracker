use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dtos::food::Consumable;

use tower_cookies::Cookies;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateConsumableError {
    #[error("Server error: {0}")]
    ServerFn(String),
}

impl From<ServerFnError> for CreateConsumableError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

impl AsStatusCode for CreateConsumableError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            CreateConsumableError::ServerFn(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/api/v1/consumables", cookies: Cookies)]
pub async fn create_consumable(data: Consumable) -> Result<(), CreateConsumableError> {
    println!("Consumable hi");
    Ok(())
}
