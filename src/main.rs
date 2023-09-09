use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router};
use axum::routing::{get};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello)
    );

    let addr = "[::]:8080".parse().unwrap();
    println!("--> Listening on {addr}\n");
    axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    Html("Hello <strong>World!!</strong>")
}