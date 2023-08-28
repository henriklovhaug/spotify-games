use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{spotify::types::Games, store::Store};

pub mod six_minutes;

pub async fn start_game(State(store): State<Store>, Path(game): Path<Games>) -> StatusCode {
    store.start_game(game).await;
    StatusCode::OK
}

pub async fn stop_game(State(store): State<Store>) -> StatusCode {
    store.end_game().await;
    StatusCode::OK
}
