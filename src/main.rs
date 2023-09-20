mod controller;
mod error;
mod model;
mod routes;

use std::net::SocketAddr;

use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING ON {addr}\n");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to initialize the Movies API server");
}
