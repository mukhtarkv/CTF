// Import from our own library
use axum::{
    Router,
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
};
use ctf_backend::{hello::routes_hello, room::routes_room, state};
use std::{borrow::Cow, sync::Arc, time::Duration};
use tokio::{net::TcpListener, signal};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let shared_state = state::SharedState::default();
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let app = Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http()),
        )
        .merge(routes_hello())
        .merge(routes_room())
        .with_state(Arc::clone(&shared_state))
        .layer(Extension(shutdown_rx.clone()))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());

    let shutdown_signal = async move {
        signal::ctrl_c().await.expect("failed to listen for ctrl-c");
        debug!("received ctrl-c, notifying websocket handlers");
        let _ = shutdown_tx.send(true);
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
