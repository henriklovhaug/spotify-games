use std::error::Error;

use reqwest::{header::CONTENT_LENGTH, Client, Response};
use serde_json::Value;

use crate::{store::Store, CLIENT};

use super::types::{CurrentSong, Song};

const CURRENTLY_PLAYING_URL: &str = "https://api.spotify.com/v1/me/player/currently-playing";

pub async fn get_current_song(store: &Store) -> Result<CurrentSong, Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let token = store.try_get_session_token().await?;

    let response = client
        .get(CURRENTLY_PLAYING_URL)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        println!(
            "Error getting current song: {:?}",
            response.text().await.unwrap()
        );
        return Err("Error getting current song".into());
    }

    parse_response_current_song(response).await
}

async fn parse_response_current_song(response: Response) -> Result<CurrentSong, Box<dyn Error>> {
    let v: Value = serde_json::from_str(&response.text().await?)?;

    let f = || "error parsing response from spotify";

    Ok(CurrentSong::new(
        v["item"]["id"].as_str().ok_or_else(f)?.to_string(),
        v["item"]["name"].as_str().ok_or_else(f)?.to_string(),
        v["item"]["artists"][0]["name"]
            .as_str()
            .ok_or_else(f)?
            .to_string(),
        v["item"]["album"]["name"]
            .as_str()
            .ok_or_else(f)?
            .to_string(),
        v["item"]["duration_ms"].as_u64().unwrap() as u32,
        v["progress_ms"].as_u64().unwrap() as u32,
        v["item"]["album"]["images"][0]["url"]
            .as_str()
            .map(|s| s.to_string()),
    ))
}

const QUEUE_URL: &str = "https://api.spotify.com/v1/me/player/queue?uri=spotify%3Atrack%3A";

pub async fn add_song_to_spotify_queue(song: Song, store: &Store) -> Result<(), Box<dyn Error>> {
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

    if !response.status().is_success() {
        let body = response.text().await?;
        println!("Added song to queue {:?}", body);
        return Err("Error adding song to queue".into());
    }
    println!("Added song to queue with status {:?}", response.status());
    Ok(())
}

const SKIP_URL: &str = "https://api.spotify.com/v1/me/player/next";

pub async fn skip(store: &Store) -> Result<(), Box<dyn Error>> {
    let token = if let Some(v) = store.get_session_token().await {
        v
    } else {
        return Err("No token found".into());
    };

    let client = CLIENT.get_or_init(Client::new);

    let response = client
        .post(SKIP_URL)
        .header(CONTENT_LENGTH, 0)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Skip music failed {:?}", response.status());
        let body = response.text().await?;
        println!("Skip music failed {:?}", body);
        return Err("Skip music failed".into());
    }

    Ok(())
}
