use crate::state::{SharedState, create_room, delete_room, get_room, list_rooms};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};
use serde::Serialize;

pub fn routes_room() -> Router<SharedState> {
    Router::new()
        .route("/rooms/{key}", post(handler_create_room))
        .route("/rooms/{key}", get(handler_get_room))
        .route("/rooms/{key}", delete(handler_delete_room))
        .route("/rooms", get(handler_list_rooms))
}

#[derive(Serialize)]
struct CreateRoomResponse {
    key: String,
    id: String,
}

async fn handler_create_room(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<CreateRoomResponse>, StatusCode> {
    match create_room(&state, &key) {
        Ok(id) => Ok(Json(CreateRoomResponse { key, id })),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

#[derive(Serialize)]
struct RoomResponse {
    key: String,
    id: String,
}

async fn handler_get_room(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<RoomResponse>, StatusCode> {
    if let Some(id) = get_room(&state, &key) {
        Ok(Json(RoomResponse { key, id }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn handler_delete_room(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    if delete_room(&state, &key) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[derive(Serialize)]
struct ListRoomsResponse {
    rooms: Vec<RoomResponse>,
}

async fn handler_list_rooms(State(state): State<SharedState>) -> Json<ListRoomsResponse> {
    let rooms = list_rooms(&state)
        .into_iter()
        .map(|(key, id)| RoomResponse { key, id })
        .collect();
    Json(ListRoomsResponse { rooms })
}
