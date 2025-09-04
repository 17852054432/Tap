use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    message: String,
}

async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "ok".to_string(),
        message: "Backend is running".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health_check))
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
