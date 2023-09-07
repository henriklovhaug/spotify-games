use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    spotify::{
        game::SINGLE_GAME_INFO,
        types::{Games, Song},
    },
    store::Store,
};

pub mod six_minutes;

pub async fn start_game(State(store): State<Store>, Path(game): Path<Games>) -> StatusCode {
    println!("Starting game: {:?}", game);
    store.start_game(game).await;
    StatusCode::OK
}

pub async fn stop_game(State(store): State<Store>) -> StatusCode {
    store.end_game().await;
    StatusCode::OK
}

pub async fn get_game_art(Path(game): Path<Games>) -> Result<Json<Song>, StatusCode> {
    let song = if let Some(song) = SINGLE_GAME_INFO.get(&game) {
        song
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(song.clone()))
}
