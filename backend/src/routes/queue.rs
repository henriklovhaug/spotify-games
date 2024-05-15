use askama::Template;
use axum::{extract::State, Form, Json};

use crate::{
    spotify::types::{QueueSong, Song},
    store::Store,
    Channel, ChannelMessage,
};

pub async fn add_to_queue_handler(
    State(store): State<Store>,
    Form(track): Form<Song>,
) -> AddedSongTemplate {
    store.add_song_to_queue(track.clone()).await;
    let sender = store.get_sender();
    let queue_song: QueueSong = track.into();
    let message = ChannelMessage::new(
        Channel::QueueSong,
        "".into(),
        Some(queue_song.get_artist().into()),
        Some(queue_song.get_name().into()),
    );
    let _ = sender.send(message);
    AddedSongTemplate {}
}

pub async fn get_queue_handler(State(store): State<Store>) -> Json<Vec<Song>> {
    let queue = store.get_song_queue().await.to_owned();
    Json(queue.into())
}

#[derive(Template)]
#[template(path = "comp/added_song.html")]
pub struct AddedSongTemplate {}
