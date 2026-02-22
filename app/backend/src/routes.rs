use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::db::Database;

type DbState = Arc<Mutex<Database>>;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Serialize)]
pub struct Entry {
    pub title: String,
    pub pubkey: String,
}

pub async fn search_entries(
    State(db): State<DbState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Entry>>, StatusCode> {
    let db = db.lock().await;

    let results = db
        .search(&params.q)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let entries: Vec<Entry> = results
        .into_iter()
        .map(|(title, pubkey)| Entry { title, pubkey })
        .collect();

    Ok(Json(entries))
}

#[derive(Deserialize)]
pub struct CreateEntry {
    pub title: String,
    pub pubkey: String,
}

pub async fn create_entry(
    State(db): State<DbState>,
    Json(payload): Json<CreateEntry>,
) -> StatusCode {
    let db = db.lock().await;

    match db.insert(&payload.title, &payload.pubkey) {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
