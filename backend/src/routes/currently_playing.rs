use std::error::Error;

use axum::{extract::State, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{store::Store, CLIENT};

#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    name: String,
    artist: String,
    album: String,
    duration: u32,
    is_playing: bool,
}

pub async fn get_currently_playing(State(store): State<Store>) -> Result<Json<Song>, String> {
    let song = get_song_spotify(store).await.map_err(|e| e.to_string())?;

    Ok(Json(song))
}

const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/me/player/currently-playing";

async fn get_song_spotify(store: Store) -> Result<Song, Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let token = store.get_session_token().await;

    let response = client
        .get(CURRENTLY_PLAYING_URL)
        .header("Authorization", format!("Bearer {}", token.unwrap()))
        .send()
        .await?;

    Ok(response.json::<Song>().await?)
}
