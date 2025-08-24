use crate::error::Error;
use crate::error::Result as AppResult;
use crate::game::Move as GameMove;
use crate::state::{
    SharedState, add_player, add_ws_sender, broadcast_to_room, create_room, delete_room,
    get_players_state, get_room, list_rooms, update_player_state,
};
use axum::{
    Router,
    extract::{
        Extension, Path, Query, State,
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

type HttpResult<T> = std::result::Result<Json<T>, StatusCode>;

pub fn routes_room() -> Router<SharedState> {
    Router::new()
        .route("/rooms", post(handler_create_room))
        .route("/rooms/{room_id}/join", get(handler_get_room))
        .route("/rooms/{room_id}", delete(handler_delete_room))
        .route("/rooms", get(handler_list_rooms))
        .route("/ws/rooms/{room_id}", get(ws_handler))
}

#[derive(Serialize)]
struct CreateRoomResponse {
    room_id: String,
    id: String,
}

#[derive(Serialize)]
struct RoomResponse {
    room_id: String,
    id: String,
}

#[derive(Serialize)]
struct ListRoomsResponse {
    rooms: Vec<RoomResponse>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientEvent {
    StartGame {},
    Chat { content: String },
    Move { dx: i32, dy: i32 },
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerEvent {
    Welcome {
        role: String,
        session_id: String,
        room: String,
    },
    UserJoined {
        session_id: String,
        player_id: i32,
    },
    UserLeft {
        session_id: String,
    },
    HostLeft {
        session_id: String,
    },
    GameStarted {
        started_by: String,
    },
    Chat {
        from: String,
        content: String,
    },
}

async fn handler_create_room(State(state): State<SharedState>) -> HttpResult<CreateRoomResponse> {
    let mut rng = rand::rng();
    let room_id;
    let id;
    debug!("Attempting to create a room");
    loop {
        let candidate = format!("{:06}", rng.random_range(0..1_000_000));
        match create_room(&state, &candidate) {
            Ok(generated_id) => {
                room_id = candidate;
                id = generated_id;
                break;
            }
            Err(_) => continue,
        }
    }
    debug!("Created a room with room_id={} and id={}", room_id, id);
    Ok(Json(CreateRoomResponse { room_id, id }))
}

async fn handler_get_room(
    Path(room_id): Path<String>,
    State(state): State<SharedState>,
) -> AppResult<Json<RoomResponse>> {
    if let Some(id) = get_room(&state, &room_id) {
        Ok(Json(RoomResponse { room_id, id }))
    } else {
        Err(Error::RoomNotFound)
    }
}

async fn handler_delete_room(
    Path(room_id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    if delete_room(&state, &room_id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn handler_list_rooms(State(state): State<SharedState>) -> Json<ListRoomsResponse> {
    let rooms = list_rooms(&state)
        .into_iter()
        .map(|(room_id, id)| RoomResponse { room_id, id })
        .collect();
    Json(ListRoomsResponse { rooms })
}

async fn ws_handler(
    Path(room_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Extension(shutdown_rx): Extension<tokio::sync::watch::Receiver<bool>>,
) -> impl IntoResponse {
    debug!(
        "ws_handler: incoming websocket upgrade for room room_id={} params={:?}",
        room_id, params
    );
    if get_room(&state, &room_id).is_none() {
        debug!(
            "ws_handler: room {} not found, existing rooms: {:?}",
            room_id,
            list_rooms(&state)
        );
        return Error::RoomNotFound.into_response();
    }
    ws.on_upgrade(move |socket| async move {
        let shutdown_rx = shutdown_rx.clone();
        debug!("ws_handler: upgrade successful for room {}", room_id);
        handle_socket(socket, room_id, params, state, shutdown_rx).await;
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    room_id: String,
    params: HashMap<String, String>,
    state: SharedState,
    mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
) {
    let role = params
        .get("role")
        .map(|s| s.as_str())
        .unwrap_or("player")
        .to_string();
    let session_id = Uuid::new_v4().to_string();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    add_ws_sender(&state, &room_id, tx);

    // Send structured welcome event
    let welcome_event = ServerEvent::Welcome {
        role: role.clone(),
        session_id: session_id.clone(),
        room: room_id.clone(),
    };
    if socket
        .send(Message::text(
            serde_json::to_string(&welcome_event).unwrap(),
        ))
        .await
        .is_err()
    {
        return;
    }

    // Notify others that a player joined
    let player_id = if role == "player" {
        let player_id = add_player(&state, &room_id);
        let joined = ServerEvent::UserJoined {
            session_id: session_id.clone(),
            player_id,
        };

        broadcast_to_room(&state, &room_id, &serde_json::to_string(&joined).unwrap());
        player_id
    } else {
        0
    };

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                if socket.send(Message::text(msg)).await.is_err() { break; }
            }
            result = socket.recv() => {
                match result {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientEvent>(&text) {
                            Ok(ClientEvent::StartGame {}) => {
                                if role == "host" {
                                    let game_started = ServerEvent::GameStarted { started_by: session_id.clone() };
                                    broadcast_to_room(&state, &room_id, &serde_json::to_string(&game_started).unwrap());
                                    // TODO: add game loop logic here
                                    // game logic needs to broadcast to room new state
                                    // use get_players_state function
                                }
                            }
                            Ok(ClientEvent::Chat { content }) => {
                                let chat = ServerEvent::Chat { from: session_id.clone(), content };
                                broadcast_to_room(&state, &room_id, &serde_json::to_string(&chat).unwrap());
                            }
                            Ok(ClientEvent::Move {dx, dy}) => {
                              let new_move = GameMove::new(dx, dy);
                              update_player_state(&state, &room_id, player_id, new_move);
                              let players_state = get_players_state(&state, &room_id);
                              println!("Players state: {:?}", players_state);
                            }
                            Err(_) => {
                                // Fallback: echo as chat
                                let chat = ServerEvent::Chat { from: session_id.clone(), content: text.to_string() };
                                broadcast_to_room(&state, &room_id, &serde_json::to_string(&chat).unwrap());
                            }
                        }
                    }
                    Some(Ok(Message::Ping(payload))) => { let _ = socket.send(Message::Pong(payload)).await; }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                    None => break,
                }
            }
            // New graceful shutdown branch
            Ok(_) = shutdown_rx.changed() => {
                let _ = socket.send(Message::Close(Some(CloseFrame { code: axum::extract::ws::close_code::NORMAL, reason: "server shutting down".into() }))).await;
                break;
            }
        }
    }

    // Connection is dropping; notify others based on role.
    if role == "player" {
        let left = ServerEvent::UserLeft {
            session_id: session_id.clone(),
        };
        broadcast_to_room(&state, &room_id, &serde_json::to_string(&left).unwrap());
    } else if role == "host" {
        let left = ServerEvent::HostLeft {
            session_id: session_id.clone(),
        };
        broadcast_to_room(&state, &room_id, &serde_json::to_string(&left).unwrap());
    }
    // Connection cleanup of dead senders happens lazily on next broadcast.
}
