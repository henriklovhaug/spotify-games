use std::error::Error;

use axum::extract::State;
use reqwest::Client;
use serde::Serialize;

use crate::{store::Store, CLIENT};

const URL: &str = "https://api.spotify.com/v1/me/player/play";

#[derive(Debug, Serialize)]
struct SkipBody {
    context_uri: String,
}

impl SkipBody {
    fn new() -> Self {
        Self {
            context_uri: "spotify:album:5ht7ItJgpBH7W6vJ5BqpPr".to_string(),
        }
    }
}

pub async fn skip_n_tracks_handler(State(store): State<Store>, body: String) {
    let n: u8 = body.parse().unwrap(); // Handle error

    if let Err(e) = skip(n, store).await {
        println!("Error: {:?}", e);
    }
}

async fn skip(_number: u8, store: Store) -> Result<(), Box<dyn Error>> {
    let token = if let Some(v) = store.get_session_token().await {
        v
    } else {
        return Err("No token found".into());
    };

    let client = CLIENT.get_or_init(Client::new);

    let body = SkipBody::new();

    let response = client
        .put(URL)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        println!("Skip music failed {:?}", body);
        return Err("Skip music failed".into());
    }

    Ok(())
}
