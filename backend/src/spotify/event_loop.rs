use chrono::Duration;
use tokio::time::sleep;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, get_current_song},
        types::SpotifyActivity,
    },
    store::Store,
};

const ADD_TO_QUEUE_THRESHOLD: i64 = 10;

pub async fn spotify_loop(store: Store) {
    loop {
        sleep(Duration::seconds(1).to_std().unwrap()).await;
        let gamestate = store.get_activity().await;

        match gamestate {
            SpotifyActivity::Music => {
                if store.view_next_song().await.is_some() {
                    let song = get_current_song(store.clone()).await.unwrap();
                    let duration_left = song.get_remaining_time().num_seconds();
                    println!("Duration left: {}", duration_left);
                    if duration_left < ADD_TO_QUEUE_THRESHOLD {
                        let next_song = store.get_next_song().await.unwrap();
                        if let Err(e) = add_song_to_spotify_queue(next_song, store.clone()).await {
                            println!("Error adding song to queue: {}", e);
                        }
                    }
                }
            }
            SpotifyActivity::Game => todo!(),
        }
    }
}
