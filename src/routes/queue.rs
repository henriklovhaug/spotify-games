use std::collections::VecDeque;

use askama::Template;
use axum::{extract::State, Form};

use crate::{spotify::types::Song, store::Store};

pub async fn add_to_queue_handler(
    State(store): State<Store>,
    Form(track): Form<Song>,
) -> AddedSongTemplate {
    store.add_song_to_queue(track.clone()).await;
    AddedSongTemplate {}
}

pub async fn get_queue_handler(State(store): State<Store>) -> SongQueueTemplate {
    let queue = store.get_song_queue().await.to_owned();
    SongQueueTemplate::new(queue)
}

#[derive(Template)]
#[template(path = "comp/added_song.html")]
pub struct AddedSongTemplate {}

#[derive(Template)]
#[template(path = "comp/queue.html")]
pub struct SongQueueTemplate {
    songs: Vec<Song>,
}

impl SongQueueTemplate {
    pub fn new(songs: VecDeque<Song>) -> Self {
        SongQueueTemplate {
            songs: songs.into_iter().collect()
        }
    }
}
