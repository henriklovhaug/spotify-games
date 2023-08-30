use std::error::Error;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, skip},
        types::Song,
    },
    store::Store,
};

//TODO change this to use the music queue
pub async fn play_opus(store: &Store) -> Result<(), Box<dyn Error>> {
    let song = Song::new(
        "3v2oAQomhOcYCPPHafS3KV",
        "Opus",
        "Eric Prydz",
        "",
        543453,
        None,
    );

    add_song_to_spotify_queue(song, store).await?;
    skip(store).await?;

    store.end_game().await;

    Ok(())
}
