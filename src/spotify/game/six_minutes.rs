use std::error::Error;

use chrono::Duration;
use reqwest::Client;
use serde::Serialize;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::{
    spotify::{
        api::get_current_song,
        types::{Games, SpotifyActivity},
    },
    store::Store,
    CLIENT,
};

pub async fn play_sixminutes(store: &Store) {
    if start_playlist(store).await.is_err() {
        store.end_game().await;
        return;
    }
    six_minutes_timer(store).await;
    store.end_game().await;
    info!("Done with game");
}

async fn six_minutes_timer(store: &Store) {
    let store_clone = store.clone();
    let mut handle = tokio::spawn(async move {
        notify_song(store_clone).await;
    });

    let mut sleep_handle = tokio::spawn(async move {
        sleep(Duration::minutes(6).to_std().unwrap()).await;
    });

    tokio::select! {
        _r_va = &mut handle => {
            warn!("Song ended early");
            sleep_handle.abort();
        },
        _r_vb = &mut sleep_handle => {
            info!("Game ended on time");
            handle.abort();
        }
    }

    let tx = store.get_sender();
    let _ = tx.send("PUT YOUR STUFF HERE".into());
}

async fn notify_song(store: Store) {
    loop {
        if store.get_activity().await != SpotifyActivity::Game(Games::SixMinutes) {
            info!("Game ended");
            break;
        }
        let tx = store.get_sender();
        sleep(Duration::seconds(1).to_std().unwrap()).await;
        let song = match get_current_song(&store).await {
            Ok(song) => song,
            Err(e) => {
                error!("Error getting current song: {:?}", e);
                continue;
            }
        };

        if let Err(e) = tx.send("PUT YOUR STUFF HERE".into()) {
            println!("Error sending message: {:?}", e);
        }
    }
}

#[derive(Serialize)]
struct PlayListBody {
    context_uri: String,
}

const PLAY_URL: &str = "https://api.spotify.com/v1/me/player/play";
const PLAYLIST_ID: &str = "spotify:playlist:6gegGeB5zoYZ0cboKww43s?si=a314b2fea15b4b93";

async fn start_playlist(store: &Store) -> Result<(), Box<dyn Error>> {
    let token = store.try_get_session_token().await?;
    let client = CLIENT.get_or_init(Client::new);

    let body = PlayListBody {
        context_uri: PLAYLIST_ID.into(),
    };

    let response = client
        .put(PLAY_URL)
        .json(&body)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        error!(
            "Error starting playlist: {:?}",
            response.text().await.unwrap()
        );
        return Err("Error starting playlist".into());
    }

    Ok(())
}
