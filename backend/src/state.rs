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

    // Find first available slot from 0-3
    let mut available_id = None;
    let order = guard.room_players.entry(room_key.to_string()).or_default();
    for id in 0..4 {
        if !order.contains(&id) {
            available_id = Some(id);
            break;
        }
    }

    match available_id {
        Some(id) => {
            // Add to room state
            let room = guard.room_state.entry(room_key.to_string()).or_default();
            room.insert(id, Move::Stay);

            // Add to player order
            let order = guard.room_players.entry(room_key.to_string()).or_default();
            order.push(id);

            id
        }
        None => {
            // Room is full, return -1 to indicate failure
            -1
        }
    }
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

pub fn remove_player(state: &SharedState, room_key: &str, player_id: i32) {
    let mut guard = state.write().unwrap();

    // Remove from room_state
    if let Some(room) = guard.room_state.get_mut(room_key) {
        room.remove(&player_id);
    }

    // Remove from room_players order
    if let Some(order) = guard.room_players.get_mut(room_key) {
        order.retain(|&id| id != player_id);
    }
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

            // Re-lock to mutate the game and snapshot positions, check for score reset
            let (positions_json_opt, should_reset_moves, players_to_reset_moves) = {
                let mut guard = state_cloned.write().unwrap();
                if let Some(game) = guard.room_game.get_mut(&room_key_string) {
                    let old_scores = game.get_scores();
                    let players_to_reset_moves = game.step(moves_arr);
                    let new_scores = game.get_scores();

                    // Check if score changed (someone scored)
                    let score_changed = old_scores != new_scores;

                    // Reset moves for players who were caught in enemy territory
                    let positions = game.positions();
                    let flag_captors = game.get_flag_captors(); // We need to add this method
                    let payload = serde_json::json!({
                        "type": "positions",
                        "players": positions,
                        "flag_captors": flag_captors,
                        "scores": game.get_scores(), // We need to add this method too
                    });
                    (
                        Some(payload.to_string()),
                        score_changed,
                        players_to_reset_moves,
                    )
                } else {
                    (None, false, Vec::new())
                }
            };

            // Reset all player moves to Stay if someone scored
            if should_reset_moves {
                let mut guard = state_cloned.write().unwrap();
                if let Some(room_state) = guard.room_state.get_mut(&room_key_string) {
                    for (_, player_move) in room_state.iter_mut() {
                        *player_move = Move::Stay;
                    }
                }
                println!("🔄 All player moves reset to Stay after score!");
            }

            // Reset moves for players who were caught in enemy territory (outside the main lock)
            if !players_to_reset_moves.is_empty() {
                let mut guard = state_cloned.write().unwrap();

                // First get the order mapping
                let order = guard
                    .room_players
                    .get(&room_key_string)
                    .cloned()
                    .unwrap_or_default();

                // Then reset moves for each player
                for &player_index in &players_to_reset_moves {
                    if let Some(&player_id) = order.get(player_index) {
                        if let Some(room_state) = guard.room_state.get_mut(&room_key_string) {
                            room_state.insert(player_id, Move::Stay);
                            println!(
                                "🔄 Reset move to Stay for player {} (index {})",
                                player_id, player_index
                            );
                        }
                    }
                }
            }

            // Broadcast after lock is released
            if let Some(json) = positions_json_opt {
                broadcast_to_room(&state_cloned, &room_key_string, &json);
            }
        }
    });

    guard.room_tasks.insert(room_key.to_string(), handle);
}
