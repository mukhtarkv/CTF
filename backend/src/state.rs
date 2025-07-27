use axum::body::Bytes;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

/// KV-get: returns Some(value) or None
pub fn kv_get(state: &SharedState, key: &str) -> Option<Bytes> {
    let guard = state.read().unwrap();
    guard.db.get(key).cloned()
}

/// KV-set: inserts or overwrites
pub fn kv_set(state: &SharedState, key: String, value: Bytes) {
    let mut guard = state.write().unwrap();
    guard.db.insert(key, value);
}

/// list-keys
pub fn list_keys(state: &SharedState) -> Vec<String> {
    let guard = state.read().unwrap();
    guard.db.keys().cloned().collect()
}
