use axum::Router;
use tokio::net::TcpListener;

use crate::hello::routes_hello;

pub use self::error::{Error, Result};

mod error;
mod hello;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new().merge(routes_hello());

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

