use std::error::Error;

use axum::{extract::State, Json};
use reqwest::{Client, Response};
use serde_json::Value;

use crate::{spotify::types::Song, store::Store, CLIENT};

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

    Ok(Song::new(
        v["item"]["id"].to_string(),
        v["item"]["name"].to_string(),
        v["item"]["artists"][0]["name"].to_string(),
        v["item"]["album"]["name"].to_string(),
        v["progress_ms"].as_u64().unwrap() as u32,
        v["is_playing"].as_bool().unwrap(),
    ))
}
