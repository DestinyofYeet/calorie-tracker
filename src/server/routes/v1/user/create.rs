use dioxus::{fullstack::AsStatusCode, prelude::*};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "server")]
use crate::server::database::DATABASE;

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
impl From<diesel::result::Error> for CreateUserError {
    fn from(value: diesel::result::Error) -> Self {
        error!("Database error: {}", value.to_string());
        Self::DatabaseError
    }
}

#[cfg(feature = "server")]
impl From<argon2::password_hash::Error> for CreateUserError {
    fn from(value: argon2::password_hash::Error) -> Self {
        error!("Failed to hash password: {}", value.to_string());
        Self::PasswordHash
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

    let mut db = match DATABASE.get().get() {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to get Database lock: {e}");
            return Err(CreateUserError::GetDatabase);
        }
    };

    {
        use crate::server::database::schema::users::{self, dsl::*};
        use diesel::insert_into;
        use diesel::prelude::*;

        let result = users::table
            .select(users::id)
            .filter(users::email.eq(&provided_email))
            .load::<i32>(&mut db)?;

        if !result.is_empty() {
            return Err(CreateUserError::EmailExists);
        }

        insert_into(users)
            .values((
                name.eq(&provided_username),
                email.eq(&provided_email),
                hashed_password.eq(&hashed_provided_password),
            ))
            .execute(&mut db)?;

        Ok(())
    }
}
