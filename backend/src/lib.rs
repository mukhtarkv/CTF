// Library interface for ctf-backend
// This allows examples and other crates to import our types

pub use self::error::{Error, Result};

pub mod error;
pub mod game;
pub mod hello;
pub mod room;
pub mod state;
