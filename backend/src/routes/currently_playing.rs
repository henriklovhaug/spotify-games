use std::error::Error;

use axum::{extract::State, Json};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

    parse_response(response).await
}

async fn parse_response(response: Response) -> Result<Song, Box<dyn Error>> {
    let v: Value = serde_json::from_str(&response.text().await?)?;

    Ok(Song {
        name: v["item"]["name"].to_string(),
        artist: v["item"]["artists"][0]["name"].to_string(),
        album: v["item"]["album"]["name"].to_string(),
        duration: v["progress_ms"].as_u64().unwrap() as u32,
        is_playing: v["is_playing"].as_bool().unwrap(),
    })
}
