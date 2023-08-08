use axum::{extract::State, Json};

use crate::{spotify::types::Song, store::Store};

pub async fn add_to_queue_handler(State(store): State<Store>) -> () {
    todo!("implement add_to_queue_handler")
}

pub async fn get_queue_handler(State(store): State<Store>) -> Json<Vec<Song>> {
    Json(Vec::new())
}
