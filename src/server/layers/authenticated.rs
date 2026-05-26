use dioxus::{
    fullstack::{extract::Request, response::Response},
    prelude::StatusCode,
    server::axum::middleware::Next,
};
use django_rs::{
    chrono::Utc, models::search::SearchQuery, server::database_strategy::DatabaseStrategy,
};
use tracing::{debug, error, trace};

use crate::server::{
    database::models::{login_token::LoginToken, user::User},
    entry::SERVER,
};

static PUBLIC_LIST_STARTS_WITH: [&str; 2] = ["/wasm/", "/assets/"];

static PUBLIC_LIST_EXACT: [&str; 5] = [
    "/",
    "/user/login",
    "/user/create",
    "/api/v1/user/login",
    "/api/v1/user/create",
];

pub async fn run_authenticated_layer(request: Request, next: Next) -> Response {
    debug!("path: {}", request.uri().path());
    if PUBLIC_LIST_STARTS_WITH
        .iter()
        .any(|item| request.uri().path().starts_with(item))
    {
        return next.run(request).await;
    }

    if PUBLIC_LIST_EXACT
        .iter()
        .any(|item| request.uri().path() == *item)
    {
        return next.run(request).await;
    }

    let db = SERVER.get_database();

    let invalid_token_header_resp = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid token header".into())
        .unwrap();

    let header = match request.headers().get("token").map(|e| e.to_str()) {
        Some(Ok(e)) => e,
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

    let user = match db.search_single_model::<User>(
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

    println!("hi {user:?}");

    next.run(request).await
}
