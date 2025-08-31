use crate::game::{GameState, Move};
use rand::Rng;
use serde_json;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

pub type SharedState = Arc<RwLock<AppState>>;

type RoomGame = GameState<4>; // adjust N as needed (2 or 4 supported by GameState)

#[derive(Default, Debug)]
pub struct AppState {
    pub room_senders: HashMap<String, Vec<UnboundedSender<String>>>,
    pub room_state: HashMap<String, HashMap<i32, Move>>,
    pub room_game: HashMap<String, RoomGame>,
    pub room_players: HashMap<String, Vec<i32>>, // stable player-id -> index order
    pub room_tasks: HashMap<String, JoinHandle<()>>, // running tick loops per room
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
            guard.room_players.insert(room_key.to_string(), Vec::new());
            guard
                .room_game
                .insert(room_key.to_string(), RoomGame::new());
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

    // Scope 1: work with room_state, then end the borrow
    let new_player_id = {
        let room = guard.room_state.entry(room_key.to_string()).or_default();
        let id = room.keys().max().copied().map_or(0, |max_id| max_id + 1);
        room.insert(id, Move::Stay);
        id
    }; // <-- 'room' (&mut ...) dropped here

    // Scope 2: now it's safe to mutably borrow room_players
    {
        let order = guard.room_players.entry(room_key.to_string()).or_default();
        if !order.contains(&new_player_id) {
            order.push(new_player_id);
        }
    } // <-- 'order' dropped here

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

pub fn ensure_room_loop(state: &SharedState, room_key: &str) {
    let mut guard = state.write().unwrap();

    // Ensure a game exists for the room
    guard
        .room_game
        .entry(room_key.to_string())
        .or_insert_with(|| RoomGame::new());

    // If loop already running, do nothing
    if guard.room_tasks.contains_key(room_key) {
        return;
    }

    let state_cloned = Arc::clone(state);
    let room_key_string = room_key.to_string();
    let handle = tokio::spawn(async move {
        let mut ticker = time::interval(Duration::from_millis(200));
        loop {
            ticker.tick().await; // every 200ms

            // Build moves array from current room_state using room_players order
            let (order, moves_snapshot) = {
                let guard = state_cloned.read().unwrap(); // read lock is enough
                let order = guard
                    .room_players
                    .get(&room_key_string)
                    .cloned() // Vec<i32>
                    .unwrap_or_default();

                let moves_snapshot = guard
                    .room_state
                    .get(&room_key_string)
                    .cloned() // HashMap<i32, Move>  (cheap: Move is Copy)
                    .unwrap_or_default();

                (order, moves_snapshot)
            }; // <- guard dropped here, no references live

            // Build fixed-size array outside the lock
            let mut moves_arr = [Move::Stay; 4];
            for (idx, pid) in order.iter().take(4).enumerate() {
                if let Some(mv) = moves_snapshot.get(pid).copied() {
                    moves_arr[idx] = mv;
                }
            }

            // Re-lock to mutate the game and snapshot positions
            let positions_json_opt = {
                let mut guard = state_cloned.write().unwrap();
                if let Some(game) = guard.room_game.get_mut(&room_key_string) {
                    game.step(moves_arr);
                    let positions = game.positions();
                    let payload = serde_json::json!({
                        "type": "positions",
                        "players": positions,
                    });
                    Some(payload.to_string())
                } else {
                    None
                }
            };

            // Broadcast after lock is released
            if let Some(json) = positions_json_opt {
                broadcast_to_room(&state_cloned, &room_key_string, &json);
            }
        }
    });

    guard.room_tasks.insert(room_key.to_string(), handle);
}
