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
        "2VjrFvthQjw7BS8aS4VdZi".into(),
        "Rattling Bog".into(),
        "Carlyle Fraser".into(),
        "Lord of the dance".into(),
        142306,
    );

    add_song_to_spotify_queue(song, &store).await?;
    skip(&store).await?;

    store.end_game().await;

    Ok(())
}
