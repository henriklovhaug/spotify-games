use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;
use tracing::info;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, get_current_song},
        types::SpotifyActivity,
    },
    store::Store,
};

use super::{
    api::skip,
    game::{single_song::play_single_song_game, six_minutes::play_sixminutes},
    types::{CurrentSong, Games},
};

const ADD_TO_QUEUE_THRESHOLD: i64 = 10;

async fn save_immediate_song(
    enqueue_time: &DateTime<Utc>,
    current_song: Option<CurrentSong>,
    store: &Store,
) -> bool {
    if *enqueue_time + Duration::seconds(ADD_TO_QUEUE_THRESHOLD) > Utc::now() {
        match current_song {
            Some(song) => {
                store.add_song_to_queue_front(song.into()).await;
                true
            }
            None => false,
        }
    } else {
        false
    }
}

pub async fn spotify_loop(store: Store) {
    let mut enqueue_time = Utc::now();
    let mut current_song = None;
    loop {
        sleep(Duration::seconds(2).to_std().unwrap()).await;
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
                        && enqueue_time + Duration::seconds(ADD_TO_QUEUE_THRESHOLD + 1) < Utc::now()
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
                    let saved = save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    play_sixminutes(&store).await;
                    if saved {
                        if let Err(e) = skip(&store).await {
                            println!("Error skipping: {}", e);
                        }
                    }
                }
                Games::RattlingBog => {
                    let saved = save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_single_song_game(&store, Games::RattlingBog).await {
                        println!("Error playing rattling bog: {}", e);
                    }
                    if saved {
                        if let Err(e) = skip(&store).await {
                            println!("Error skipping: {}", e);
                        }
                    }
                }
                Games::Opus => {
                    let saved = save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_single_song_game(&store, Games::Opus).await {
                        println!("Error playing opus: {}", e);
                    }
                    if saved {
                        if let Err(e) = skip(&store).await {
                            println!("Error skipping: {}", e);
                        }
                    }
                }
                Games::Palmerna => {
                    let saved = save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_single_song_game(&store, Games::Palmerna).await {
                        println!("Error playing palmerna: {}", e);
                    }
                    if saved {
                        if let Err(e) = skip(&store).await {
                            println!("Error skipping: {}", e);
                        }
                    }
                }
                Games::Thunder => {
                    let saved = save_immediate_song(&enqueue_time, current_song, &store).await;
                    current_song = None;
                    if let Err(e) = play_single_song_game(&store, Games::Thunder).await {
                        println!("Error playing palmerna: {}", e);
                    }
                    if saved {
                        if let Err(e) = skip(&store).await {
                            println!("Error skipping: {}", e);
                        }
                    }
                }
            },
        }
    }
}
