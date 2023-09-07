use axum::{extract::State, http::StatusCode, Json};

use crate::{
    spotify::types::{QueueSong, Song},
    store::Store,
    Channel, ChannelMessage,
};

pub async fn add_to_queue_handler(
    State(store): State<Store>,
    Json(track): Json<Song>,
) -> StatusCode {
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
    StatusCode::CREATED
}

pub async fn get_queue_handler(State(store): State<Store>) -> Json<Vec<Song>> {
    let queue = store.get_song_queue().await.to_owned();
    Json(queue.into())
}
