use dioxus::{fullstack::AsStatusCode, prelude::*};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "server")]
use {
    crate::server::database::models::login_token::LoginToken,
    crate::server::entry::SERVER,
    django_rs::chrono::{Months, Utc},
    django_rs::{
        models::search::SearchQuery,
        server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
    },
    uuid::Uuid,
};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum LoginUserError {
    #[error("Server Error: {0}")]
    ServerFn(String),

    #[error("Email is empty!")]
    EmailEmpty,

    #[error("Password is empty!")]
    PasswordEmpty,

    #[error("Database error!")]
    Database,

    #[error("Invalid email or password")]
    InvalidEmailOrPassword,
}

impl From<ServerFnError> for LoginUserError {
    fn from(value: ServerFnError) -> Self {
        Self::ServerFn(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<DatabaseStrategyError> for LoginUserError {
    fn from(value: DatabaseStrategyError) -> Self {
        error!("Failed to query database: {value}");
        Self::Database
    }
}

impl AsStatusCode for LoginUserError {
    fn as_status_code(&self) -> StatusCode {
        match *self {
            LoginUserError::Database | LoginUserError::ServerFn(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginUserError::InvalidEmailOrPassword
            | LoginUserError::PasswordEmpty
            | LoginUserError::EmailEmpty => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/api/v1/user/login")]
pub async fn login_user(
    provided_email: String,
    provided_password: String,
) -> Result<(), LoginUserError> {
    use crate::server::database::models::user::User;
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    if provided_email.is_empty() {
        return Err(LoginUserError::EmailEmpty);
    }

    if provided_password.is_empty() {
        return Err(LoginUserError::PasswordEmpty);
    }

    let db = SERVER.get_database();

    let user = db.search_single_model::<User>(
        &db.get_connection(),
        SearchQuery::empty().add_constraint(("email", &provided_email)),
    )?;

    let user = match user {
        Some(value) => value,
        None => return Err(LoginUserError::InvalidEmailOrPassword),
    };

    let argon = Argon2::default();
    let parsed_hash = PasswordHash::new(&user.hashed_password).map_err(|e| {
        error!("Failed to parse password hash: {e}");
        LoginUserError::InvalidEmailOrPassword
    })?;

    if argon
        .verify_password(provided_password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(LoginUserError::InvalidEmailOrPassword);
    }

    // User is valid beyond this point

    let token = Uuid::new_v4().to_string();

    let offset_time = Utc::now().checked_add_months(Months::new(1)).unwrap();

    let mut token = LoginToken {
        id: None,
        user_id: user.id.unwrap(),
        token,
        expires_at: offset_time,
    };

    db.save_model(&db.get_connection(), &mut token)?;

    // let mut response = Response::new(());
    // response
    //     .headers_mut()
    //     .insert(SET_COOKIE, HeaderValue::from_str(&token.token).unwrap());

    // set_respo

    Ok(())
}
