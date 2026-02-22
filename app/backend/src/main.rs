mod db;
mod routes;

use axum::{Router, routing::get, routing::post};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use db::Database;
use routes::{create_entry, search_entries};

#[tokio::main]
async fn main() {
    let db = Database::new("entries.db").expect("Failed to initialize database");
    let db = Arc::new(Mutex::new(db));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/search", get(search_entries))
        .route("/entries", post(create_entry))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
