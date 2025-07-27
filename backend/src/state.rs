use axum::body::Bytes;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

/// Creates a new room with the given key if it does not exist, returning its unique ID or an error.
pub fn create_room(state: &SharedState, key: &str) -> Result<String, ()> {
    let mut guard = state.write().unwrap();
    if guard.db.contains_key(key) {
        Err(())
    } else {
        let id = Uuid::new_v4().to_string();
        guard.db.insert(key.to_string(), Bytes::from(id.clone()));
        Ok(id)
    }
}

/// Retrieves the ID of the room with the given key, if it exists.
pub fn get_room(state: &SharedState, key: &str) -> Option<String> {
    let guard = state.read().unwrap();
    guard
        .db
        .get(key)
        .map(|b| String::from_utf8_lossy(&b[..]).to_string())
}

/// Deletes the room with the given key, returning true if removed.
pub fn delete_room(state: &SharedState, key: &str) -> bool {
    let mut guard = state.write().unwrap();
    guard.db.remove(key).is_some()
}

/// Lists all rooms as (key, ID) pairs.
pub fn list_rooms(state: &SharedState) -> Vec<(String, String)> {
    let guard = state.read().unwrap();
    guard
        .db
        .iter()
        .map(|(k, v)| (k.clone(), String::from_utf8_lossy(&v[..]).to_string()))
        .collect()
}
