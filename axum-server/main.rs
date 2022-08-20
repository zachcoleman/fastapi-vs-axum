use axum::{extract, routing::get, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello_world", get(hello_world_handler))
        .route("/:id/:user", get(capture_and_wildcard_handler))
        .route("/json", post(json_parser_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// hello world handlder
async fn hello_world_handler() -> &'static str {
    "Hello, World!"
}

/// capture and wildcard handler
async fn capture_and_wildcard_handler(
    extract::Path((id, user)): extract::Path<(usize, String)>,
) -> String {
    format!("{} - {}", id, user)
}

#[derive(Deserialize)]
struct Payload {
    str1: String,
    str2: String,
    int1: usize,
    int2: isize,
    alphanumeric: String,
}

/// json parsing handler
async fn json_parser_handler(extract::Json(payload): extract::Json<Payload>) -> String {
    return payload.alphanumeric;
}
