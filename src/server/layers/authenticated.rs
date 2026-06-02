use dioxus::{
    fullstack::{
        extract::Request,
        response::{IntoResponse, Response},
        Redirect,
    },
    prelude::StatusCode,
    server::axum::middleware::Next,
};
use django_rs::{
    chrono::Utc, models::search::SearchQuery, server::database_strategy::DatabaseStrategy,
};
use tower_cookies::Cookies;
use tracing::error;

use crate::server::{
    database::models::{login_token::LoginToken, user::UserDB},
    entry::SERVER,
};

static WHITELIST_STARTS_WITH: [&str; 2] = ["/wasm/", "/assets/"];

static WHITELIST_EXACT: [&str; 6] = [
    "/",
    "/user/login",
    "/user/create",
    "/api/v1/user/create",
    "/api/v1/user/login",
    "/api/v1/user/is_authed",
];

pub async fn run_authenticated_layer(
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Response {
    let uri_path = request.uri().path();

    if WHITELIST_STARTS_WITH
        .iter()
        .any(|item| uri_path.starts_with(item))
    {
        return next.run(request).await;
    }

    if WHITELIST_EXACT.contains(&uri_path) {
        return next.run(request).await;
    }

    let db = SERVER.get_database();

    let invalid_token_header_resp = Redirect::temporary("/user/login").into_response();

    let header = match cookies.get("token") {
        Some(e) => e.value().to_string(),
        _ => return invalid_token_header_resp,
    };

    let token = match db
        .search_single_model::<LoginToken>(
            &db.get_connection(),
            SearchQuery::empty().add_constraint(("token", header)),
        )
        .expect("to query token")
    {
        Some(value) => value,
        None => {
            return invalid_token_header_resp;
        }
    };

    if token.expires_at < Utc::now() {
        return invalid_token_header_resp;
    }

    let user = match db.search_single_model::<UserDB>(
        &db.get_connection(),
        SearchQuery::empty().add_constraint(("id", token.user_id)),
    ) {
        Ok(value) => match value {
            Some(value) => value,
            None => return invalid_token_header_resp,
        },
        Err(e) => {
            error!("Failed to find users: {e}");
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to search users".into())
                .unwrap();
        }
    };

    request.extensions_mut().insert(user);

    next.run(request).await
}
