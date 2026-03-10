use diesel::prelude::*;

use dioxus::{
    fullstack::{extract::Request, response::Response},
    prelude::StatusCode,
    server::axum::middleware::Next,
};
use tracing::error;

use crate::server::database::{models::user::User, DATABASE};

pub async fn run_authenticated_layer(request: Request, next: Next) -> Response {
    let mut db = match DATABASE.get().get() {
        Err(e) => {
            error!("Failed to get database connection: {e}");
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to get database connection".into())
                .unwrap();
        }

        Ok(value) => value,
    };

    {
        use crate::server::database::schema::users::dsl::*;

        let results = users.select(User::as_select()).load(&mut db).expect("test");
        println!("results: {results:?}");
    }

    println!("hi");
    next.run(request).await
}
