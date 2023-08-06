use std::error::Error;

use axum::{extract::State, headers::Authorization, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{spotify::token::login, store::Store, CLIENT};

#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    name: String,
    artist: String,
    album: String,
    duration: u32,
    is_playing: bool,
}

pub async fn get_currently_playing(State(store): State<Store>) -> Result<String, String> {
    let song = get_song_spotify(store).await.map_err(|e| e.to_string())?;

    // Ok(Json(song))
    Ok(song)
}

// const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/me/player/currently-playing";
const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/me/player";
// const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/tracks/2TpxZ7JUBn3uw46aR7qd6V";

async fn get_song_spotify(store: Store) -> Result<String, Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    if !store.valid_token().await {
        login(store.clone()).await?;
    }

    let token = store.get_session_token().await;

    println!("Token: {}", token);

    let response = client
        .get(CURRENTLY_PLAYING_URL)
        .header(
            "Authorization",
            format!("Bearer {}", token),
        )
        .send()
        .await?;

    // Ok(response.json::<Song>().await?)
    //
    Ok(response.text().await?)
}
