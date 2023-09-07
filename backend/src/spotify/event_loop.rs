use chrono::{Duration, Utc, DateTime};
use tokio::time::sleep;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, get_current_song},
        types::SpotifyActivity,
    },
    store::Store,
};

use super::{
    game::{
        opus::play_opus, palmerna::play_palmerna, rattling_bog::play_rattling_bog,
        six_minutes::play_sixminutes,
    },
    types::{Games, CurrentSong}, api::skip,
};

const ADD_TO_QUEUE_THRESHOLD: i64 = 10;

async fn save_immediate_song(enqueue_time: &DateTime<Utc>, current_song: Option<CurrentSong>, store: &Store) {
    if *enqueue_time + Duration::seconds(9) < Utc::now() {
        match current_song {
            Some(song) => {
                store
                    .get_writable_song_queue()
                    .await
                    .push_front(song.into());
                if let Err(e) = skip(&store).await {
                    eprintln!("Failed to skip during grace period for enqueued song: {e}");
                };
            }
            None => {}
        }
    }
}

pub async fn spotify_loop(store: Store) {
    let mut enqueue_time = Utc::now();
    let mut current_song = None;
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
                        current_song = Some(song);
                        enqueue_time = Utc::now();
                    }
                }
            }
            // Games need to handle their own amount of time
            SpotifyActivity::Game(game) => match game {
                Games::SixMinutes => {
                    save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    play_sixminutes(&store).await
                }
                Games::RattlingBog => {
                    save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_rattling_bog(&store).await {
                        println!("Error playing rattling bog: {}", e);
                    }
                }
                Games::Opus => {
                    save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_opus(&store).await {
                        println!("Error playing opus: {}", e);
                    }
                }
                Games::Palmerna => {
                    save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_palmerna(&store).await {
                        println!("Error playing palmerna: {}", e);
                    }
                }
            },
        }
    }
}
