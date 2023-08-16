use axum::http::StatusCode;
use axum::{extract::State, Json};

use crate::{spotify::types::Song, store::Store};

pub async fn add_to_queue_handler(
    State(store): State<Store>,
    Json(track): Json<Song>,
) -> StatusCode {
    store.add_song_to_queue(track).await;
    StatusCode::CREATED
}

pub async fn get_queue_handler(State(store): State<Store>) -> Json<Vec<Song>> {
    let queue = store.get_song_queue().await;
    Json(queue)
}
