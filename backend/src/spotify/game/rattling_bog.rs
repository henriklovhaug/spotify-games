use std::error::Error;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, skip},
        types::Song,
    },
    store::Store,
};

//TODO change this to use the music queue
pub async fn play_rattling_bog(store: &Store) -> Result<(), Box<dyn Error>> {
    let song = Song::new(
        "2VjrFvthQjw7BS8aS4VdZi",
        "Rattling Bog",
        "Carlyle Fraser",
        "Lord of the dance",
        142306,
        Some("https://i.scdn.co/image/ab67616d0000b27324492f2ba3a1d995e1faf5d8"),
    );

    add_song_to_spotify_queue(song, store).await?;
    skip(store).await?;

    store.end_game().await;

    Ok(())
}
