use chrono::{Duration, Utc};
use tokio::time::sleep;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, get_current_song},
        types::SpotifyActivity,
    },
    store::Store,
};

use super::{
    game::{opus::play_opus, rattling_bog::play_rattling_bog, six_minutes::play_sixminutes},
    types::Games,
};

const ADD_TO_QUEUE_THRESHOLD: i64 = 10;

pub async fn spotify_loop(store: Store) {
    let mut enqueue_time = Utc::now();
    loop {
        sleep(Duration::seconds(1).to_std().unwrap()).await;
        let gamestate = store.get_activity().await;

        match gamestate {
            SpotifyActivity::Music => {
                if store.view_next_song().await.is_some() {
                    let song = if let Ok(song) = get_current_song(&store).await {
                        song
                    } else {
                        return;
                    };
                    let duration_left = song.get_remaining_time().num_seconds();
                    if duration_left < ADD_TO_QUEUE_THRESHOLD
                        && enqueue_time + Duration::seconds(9) < Utc::now()
                    {
                        let next_song = store.get_next_song().await.unwrap();
                        if let Err(e) = add_song_to_spotify_queue(next_song, &store).await {
                            println!("Error adding song to queue: {}", e);
                        }
                        enqueue_time = Utc::now();
                    }
                }
            }
            // Games need to handle their own amount of time
            SpotifyActivity::Game(game) => match game {
                Games::SixMinutes => play_sixminutes(&store).await,
                Games::RattlingBog => {
                    if let Err(e) = play_rattling_bog(&store).await {
                        println!("Error playing rattling bog: {}", e);
                    }
                }
                Games::Opus => {
                    if let Err(e) = play_opus(&store).await {
                        println!("Error playing opus: {}", e);
                    }
                }
            },
        }
    }
}
