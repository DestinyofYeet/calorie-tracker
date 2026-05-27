use dioxus::prelude::*;
use tower_cookies::Cookies;

use crate::server::routes::v1::user::is_authed::IsAuthedError;

#[cfg(feature = "server")]
use django_rs::{models::search::SearchQuery, server::database_strategy::DatabaseStrategy};

#[get("/api/v1/user/is_authed", cookies: Cookies)]
pub async fn is_user_authed() -> Result<bool, IsAuthedError> {
    use crate::server::database::models::login_token::LoginToken;
    use crate::server::entry::SERVER;

    let token = match cookies.get("token") {
        Some(value) => value,
        None => return Ok(false),
    };

    let db = SERVER.get_database();

    match db.search_single_model::<LoginToken>(
        &db.get_connection(),
        SearchQuery::empty().add_constraint(("token", token.value())),
    )? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
