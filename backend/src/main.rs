// src/main.rs
mod config;
mod models;
mod routes;
mod tmdb;
mod torrentio;

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::config::build_app_state;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    let state = build_app_state();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = create_router(state).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Nebula Backend running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
