use crate::game::Move;
use rand::Rng;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub struct AppState {
    pub room_senders: HashMap<String, Vec<UnboundedSender<String>>>,
    pub room_state: HashMap<String, HashMap<i32, Move>>,
}

/// Creates a new room with the given room_key if it does not exist, returning its unique ID or an error.
pub fn create_room(state: &SharedState) -> String {
    let mut guard = state.write().unwrap();
    let mut rng = rand::rng();
    let room_key;
    loop {
        let candidate = format!("{:06}", rng.random_range(0..1_000_000));
        if guard.room_state.contains_key(&candidate) {
            continue;
        } else {
            room_key = candidate;
            guard
                .room_state
                .insert(room_key.to_string(), HashMap::new());
            break;
        }
    }
    room_key
}

/// Retrieves the ID of the room with the given room_key, if it exists.
pub fn get_room_state(state: &SharedState, room_key: &str) -> Option<HashMap<i32, Move>> {
    let guard = state.read().unwrap();
    guard.room_state.get(room_key).cloned()
}

/// Deletes the room with the given room_key, returning true if removed.
pub fn delete_room(state: &SharedState, room_key: &str) -> bool {
    let mut guard = state.write().unwrap();
    guard.room_state.remove(room_key).is_some()
}

/// Lists all rooms as (room_key, ID) pairs.
pub fn list_rooms(state: &SharedState) -> Vec<String> {
    let guard = state.read().unwrap();
    guard.room_state.keys().cloned().collect()
}

/// Register a websocket sender for a room so we can broadcast to it later.
pub fn add_ws_sender(state: &SharedState, room_key: &str, sender: UnboundedSender<String>) {
    let mut guard = state.write().unwrap();
    guard
        .room_senders
        .entry(room_key.to_string())
        .or_default()
        .push(sender);
}

pub fn add_player(state: &SharedState, room_key: &str) -> i32 {
    let mut guard = state.write().unwrap();
    let room = guard.room_state.entry(room_key.to_string()).or_default();

    // Determine next player ID
    let new_player_id = room.keys().max().map_or(1, |max_id| *max_id + 1);

    // Insert with Move::Stay
    room.insert(new_player_id, Move::Stay);

    new_player_id
}

pub fn update_player_state(state: &SharedState, room_key: &str, player_id: i32, new_move: Move) {
    let mut guard = state.write().unwrap();
    guard
        .room_state
        .entry(room_key.to_string())
        .or_default()
        .insert(player_id, new_move);
}

pub fn get_players_state(state: &SharedState, room_key: &str) -> Option<HashMap<i32, Move>> {
    let guard = state.read().unwrap();
    guard.room_state.get(room_key).cloned()
}

/// Broadcast a text message to all active senders in the room, pruning dead ones.
pub fn broadcast_to_room(state: &SharedState, room_key: &str, msg: &str) {
    let mut guard = state.write().unwrap();
    if let Some(senders) = guard.room_senders.get_mut(room_key) {
        senders.retain(|tx| tx.send(msg.to_string()).is_ok());
    }
}
