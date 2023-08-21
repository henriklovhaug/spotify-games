use chrono::Duration;
use tokio::time::sleep;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, get_current_song},
        types::SpotifyActivity,
    },
    store::Store,
};

use super::game::{rattling_bog::play_rattling_bog, six_minutes::play_sixminutes};

const ADD_TO_QUEUE_THRESHOLD: i64 = 10;

pub async fn spotify_loop(store: Store) {
    loop {
        sleep(Duration::seconds(1).to_std().unwrap()).await;
        let gamestate = store.get_activity().await;

        match gamestate {
            SpotifyActivity::Music => {
                if store.view_next_song().await.is_some() {
                    let song = match get_current_song(&store).await {
                        Ok(song) => song,
                        Err(_) => return,
                    };
                    let duration_left = song.get_remaining_time().num_seconds();
                    if duration_left < ADD_TO_QUEUE_THRESHOLD {
                        let next_song = store.get_next_song().await.unwrap();
                        if let Err(e) = add_song_to_spotify_queue(next_song, &store).await {
                            println!("Error adding song to queue: {}", e);
                        }
                    }
                }
            }
            // Games need to handle their own amount of time
            SpotifyActivity::Game(game) => match game {
                super::types::Games::SixMinutes => play_sixminutes(&store).await,
                super::types::Games::RattlingBog => play_rattling_bog(store.clone()).await.unwrap(),
            },
        }
    }
}
