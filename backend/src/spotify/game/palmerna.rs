use std::error::Error;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, skip},
        types::Song,
    },
    store::Store,
};

//TODO change this to use the music queue
pub async fn play_palmerna(store: &Store) -> Result<(), Box<dyn Error>> {
    let song = Song::new(
        "5hsZ6loP0rseyjleWs0cZ1",
        "Der Palmane Bor",
        "Medena",
        "",
        225226,
        Some("https://i.scdn.co/image/ab67616d0000b2732a73b3592817536ffa7217c9"),
    );

    add_song_to_spotify_queue(song, store).await?;
    skip(store).await?;

    store.end_game().await;

    Ok(())
}
