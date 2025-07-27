use crate::state::{SharedState, kv_get, kv_set, list_keys};
use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::Serialize;

#[derive(Serialize)]
struct KeysResponse {
    keys: Vec<String>,
}

pub fn routes_kv() -> Router<SharedState> {
    Router::new()
        .route("/kv/{key}", get(handler_kv_get))
        .route("/kv/{key}", post(handler_kv_set))
        .route("/kv", get(handler_list_keys))
}

async fn handler_kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, StatusCode> {
    kv_get(&state, &key).ok_or(StatusCode::NOT_FOUND)
}

async fn handler_kv_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    body: Bytes,
) -> StatusCode {
    kv_set(&state, key, body);
    StatusCode::CREATED
}

async fn handler_list_keys(State(state): State<SharedState>) -> Json<KeysResponse> {
    let keys = list_keys(&state);
    Json(KeysResponse { keys })
}
