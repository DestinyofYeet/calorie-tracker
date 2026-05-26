use dioxus::{fullstack::AsStatusCode, prelude::*};
#[cfg(feature = "server")]
use django_rs::server::database_strategy::DatabaseStrategyError;
#[cfg(feature = "server")]
use django_rs::{models::search::SearchQuery, server::database_strategy::DatabaseStrategy};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateUserError {
    #[error("Failed to get database!")]
    GetDatabase,

    #[error("Empty username!")]
    UsernameEmpty,

    #[error("Empty email!")]
    EmailEmpty,

    #[error("This email is already used!")]
    EmailExists,

    #[error("This email is invalid!")]
    EmailInvalid,

    #[error("Empty password!")]
    PasswordEmpty,

    #[error("Failed to hash password!")]
    PasswordHash,

    #[error("Database error!")]
    DatabaseError,

    #[error("Server Error: {0}")]
    ServerFn(String),
}

impl From<ServerFnError> for CreateUserError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<argon2::password_hash::Error> for CreateUserError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("Failed to hash password: {}", value.to_string());
        Self::PasswordHash
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for CreateUserError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Failed to query database: {value}");
        Self::DatabaseError
    }
}

impl AsStatusCode for CreateUserError {
    fn as_status_code(&self) -> StatusCode {
        match *self {
            CreateUserError::UsernameEmpty
            | CreateUserError::EmailEmpty
            | CreateUserError::EmailInvalid
            | CreateUserError::PasswordEmpty
            | CreateUserError::EmailExists => StatusCode::BAD_REQUEST,
            CreateUserError::GetDatabase
            | CreateUserError::DatabaseError
            | CreateUserError::PasswordHash
            | CreateUserError::ServerFn(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/api/v1/user/create")]
pub async fn create_user(
    provided_username: String,
    provided_email: String,
    provided_password: String,
) -> Result<(), CreateUserError> {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::Argon2;
    use argon2::PasswordHasher;

    use crate::server::database::models::user::User;
    use crate::server::entry::SERVER;

    if provided_username.is_empty() {
        return Err(CreateUserError::UsernameEmpty);
    }

    if provided_email.is_empty() {
        return Err(CreateUserError::EmailEmpty);
    }

    if provided_password.is_empty() {
        return Err(CreateUserError::PasswordEmpty);
    }

    // very rudamentary email checking
    if !provided_email.contains("@") || !provided_email.contains(".") {
        return Err(CreateUserError::EmailInvalid);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_provided_password = argon2
        .hash_password(provided_password.as_bytes(), &salt)?
        .to_string();

    let db = SERVER.get_database();

    let exists = db.search_single_model::<User>(
        &db.get_connection(),
        SearchQuery::empty().add_constraint(("email", &provided_username)),
    )?;

    if exists.is_some() {
        return Err(CreateUserError::EmailExists);
    }

    let mut model = User {
        id: None,
        name: provided_username,
        email: provided_email,
        hashed_password: hashed_provided_password,
    };

    db.save_model(&db.get_connection(), &mut model)?;

    Ok(())
}
