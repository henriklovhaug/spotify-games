use std::error::Error;

use axum::{
    extract::{Query, State},
    Json,
};
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::Value;

use crate::{spotify::types::Song, store::Store, CLIENT};

#[derive(Debug, Deserialize)]
pub struct Params {
    search: String,
}

const URL: &str = "https://api.spotify.com/v1/search?type=track&limit=10&q=";

pub async fn search_song_handler(
    State(store): State<Store>,
    Query(param): Query<Params>,
) -> Result<Json<Vec<Song>>, String> {
    let search_str = format!("{}{}", URL, param.search);
    let token = store
        .get_session_token()
        .await
        .ok_or("No session token set")?;

    let songs = search(&search_str, &token)
        .await
        .map_err(|err| format!("Could not finish request: {}", err.to_string()))?;
    Ok(Json(songs))
}

async fn search(search_str: &str, auth: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let response = client.get(search_str).bearer_auth(auth).send().await?;

    parse_response(response).await
}

async fn parse_response(response: Response) -> Result<Vec<Song>, Box<dyn Error>> {
    let parsed: Value = serde_json::from_str(&response.text().await?)?;
    let items = parsed["tracks"]["items"].as_array().ok_or("parse error")?;
    let mut songs = Vec::new();
    for song in items.iter() {
        let id = song["id"].to_string();
        let name = song["name"].to_string();
        let artist = song["artists"][0]["name"].to_string();
        let album = song["album"]["name"].to_string();
        let duration = song["duration_ms"].as_u64().unwrap() as u32;
        let is_playing = false;
        let song = Song::new(id, name, artist, album, duration, is_playing);
        songs.push(song);
    }
    Ok(songs)
}
