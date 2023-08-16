use std::error::Error;

use reqwest::{header::CONTENT_LENGTH, Client, Response};
use serde_json::Value;

use crate::{store::Store, CLIENT};

use super::types::{CurrentSong, Song};

const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/me/player/currently-playing";

pub async fn get_current_song(store: Store) -> Result<CurrentSong, Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let token = store.get_session_token().await;

    let response = client
        .get(CURRENTLY_PLAYING_URL)
        .header("Authorization", format!("Bearer {}", token.unwrap()))
        .send()
        .await?;

    parse_response(response).await
}

async fn parse_response(response: Response) -> Result<CurrentSong, Box<dyn Error>> {
    let v: Value = serde_json::from_str(&response.text().await?)?;

    Ok(CurrentSong::new(
        v["item"]["id"].to_string(),
        v["item"]["name"].to_string(),
        v["item"]["artists"][0]["name"].to_string(),
        v["item"]["album"]["name"].to_string(),
        v["item"]["duration_ms"].as_u64().unwrap() as u32,
        v["progress_ms"].as_u64().unwrap() as u32,
    ))
}

const QUEUE_URL: &str = "https://api.spotify.com/v1/me/player/queue?uri=spotify%3Atrack%3A";

pub async fn add_song_to_spotify_queue(song: Song, store: Store) -> Result<(), Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);
    let url = format!("{}{}", QUEUE_URL, song.get_id());
    let token = store
        .get_session_token()
        .await
        .ok_or("No session token set")?;

    let response = client
        .post(&url)
        .header(CONTENT_LENGTH, 0)
        .bearer_auth(token)
        .send()
        .await?;
    if response.status() != 204 {
        return Err("Error adding song to queue".into());
    }
    Ok(())
}
