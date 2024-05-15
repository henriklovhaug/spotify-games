use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;

use crate::{
    spotify::{
        game::SINGLE_GAME_INFO,
        types::{Games, Song},
    },
    store::Store,
};

pub mod six_minutes;

pub async fn start_game(
    State(store): State<Store>,
    Path(game): Path<Games>,
) -> GameResponseTemplate {
    info!("Starting game: {}", game);
    store.start_game(game).await;
    GameResponseTemplate {
        game: game.to_string(),
    }
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

#[derive(Template)]
#[template(path = "comp/game_started.html")]
pub struct GameResponseTemplate {
    pub game: String,
}
