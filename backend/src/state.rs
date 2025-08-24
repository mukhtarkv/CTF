use crate::game::Move;
use axum::body::Bytes;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub struct AppState {
    pub db: HashMap<String, Bytes>,
    pub room_senders: HashMap<String, Vec<UnboundedSender<String>>>,
    pub room_state: HashMap<String, HashMap<i32, Move>>,
}

/// Creates a new room with the given room_id if it does not exist, returning its unique ID or an error.
pub fn create_room(state: &SharedState, room_id: &str) -> Result<String, ()> {
    let mut guard = state.write().unwrap();
    if guard.db.contains_key(room_id) {
        Err(())
    } else {
        let id = Uuid::new_v4().to_string();
        guard
            .db
            .insert(room_id.to_string(), Bytes::from(id.clone()));
        Ok(id)
    }
}

/// Retrieves the ID of the room with the given room_id, if it exists.
pub fn get_room(state: &SharedState, room_id: &str) -> Option<String> {
    let guard = state.read().unwrap();
    guard
        .db
        .get(room_id)
        .map(|b| String::from_utf8_lossy(&b[..]).to_string())
}

/// Deletes the room with the given room_id, returning true if removed.
pub fn delete_room(state: &SharedState, room_id: &str) -> bool {
    let mut guard = state.write().unwrap();
    guard.db.remove(room_id).is_some()
}

/// Lists all rooms as (room_id, ID) pairs.
pub fn list_rooms(state: &SharedState) -> Vec<(String, String)> {
    let guard = state.read().unwrap();
    guard
        .db
        .iter()
        .map(|(k, v)| (k.clone(), String::from_utf8_lossy(&v[..]).to_string()))
        .collect()
}

/// Register a websocket sender for a room so we can broadcast to it later.
pub fn add_ws_sender(state: &SharedState, room_id: &str, sender: UnboundedSender<String>) {
    let mut guard = state.write().unwrap();
    guard
        .room_senders
        .entry(room_id.to_string())
        .or_default()
        .push(sender);
}

pub fn add_player(state: &SharedState, room_id: &str) -> i32 {
    let mut guard = state.write().unwrap();
    let room = guard.room_state.entry(room_id.to_string()).or_default();

    // Determine next player ID
    let new_player_id = room.keys().max().map_or(1, |max_id| *max_id + 1);

    // Insert with Move::Stay
    room.insert(new_player_id, Move::Stay);

    new_player_id
}

pub fn update_player_state(state: &SharedState, room_id: &str, player_id: i32, new_move: Move) {
    let mut guard = state.write().unwrap();
    guard
        .room_state
        .entry(room_id.to_string())
        .or_default()
        .insert(player_id, new_move);
}

pub fn get_players_state(state: &SharedState, room_id: &str) -> Option<HashMap<i32, Move>> {
    let guard = state.read().unwrap();
    guard.room_state.get(room_id).cloned()
}

/// Broadcast a text message to all active senders in the room, pruning dead ones.
pub fn broadcast_to_room(state: &SharedState, room_id: &str, msg: &str) {
    let mut guard = state.write().unwrap();
    if let Some(senders) = guard.room_senders.get_mut(room_id) {
        senders.retain(|tx| tx.send(msg.to_string()).is_ok());
    }
}
