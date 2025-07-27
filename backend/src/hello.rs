use crate::state::SharedState;
use axum::{
    Router,
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

// NOTE: just for testing
pub fn routes_hello() -> Router<SharedState> {
    Router::new()
        .route("/good-bye", get(handler_good_bye))
        .route("/hello/{name}", get(handler_hello))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("Steve!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_good_bye() -> impl IntoResponse {
    Html(format!("Good bye!"))
}
