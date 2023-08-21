use std::error::Error;

use chrono::Duration;
use rand::Rng;
use reqwest::Client;
use serde::Serialize;
use tokio::time::sleep;

use crate::{store::Store, ChannelMessage, CLIENT};

pub async fn play_sixminutes(store: Store) {
    if let Err(e) = start_playlist(&store).await {
        println!("Error starting playlist: {:?}", e);
    }
    six_minutes_timer(store).await;
}

async fn six_minutes_timer(store: Store) -> () {
    let message = ChannelMessage::new("six minutes".into(), "Game over".into());
    let tx = store.get_sender();

    sleep(Duration::minutes(6).to_std().unwrap()).await;
    let _ = tx.send(message);
}

#[derive(Serialize)]
struct PlayListBody {
    context_uri: String,
    offset: Option<u32>,
    position_ms: Option<u32>,
}

const PLAY_URL: &str = "https://api.spotify.com/v1/me/player/play";
const PLAYLIST_ID: &str = "spotify:playlist:6gegGeB5zoYZ0cboKww43s?si=a314b2fea15b4b93";

async fn start_playlist(store: &Store) -> Result<(), Box<dyn Error>> {
    let token = store.try_get_session_token().await?;
    let client = CLIENT.get_or_init(Client::new);
    // let mut rng = rand::thread_rng();
    //
    // let offset = rng.gen_range(0..100);
    //
    // drop(rng);

    let body = PlayListBody {
        context_uri: PLAYLIST_ID.into(),
        offset: None,
        position_ms: None,
    };

    let response = client
        .put(PLAY_URL)
        .json(&body)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        println!(
            "Error starting playlist: {:?}",
            response.text().await.unwrap()
        );
        return Err("Error starting playlist".into());
    }

    Ok(())
}
