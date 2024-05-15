use std::error::Error;

use crate::{
    spotify::{
        api::{add_song_to_spotify_queue, skip},
        types::Games,
    },
    store::Store,
};

use super::SINGLE_GAME_INFO;

//TODO change this to use the music queue
pub async fn play_single_song_game(store: &Store, game: Games) -> Result<(), Box<dyn Error>> {
    let song = if let Some(song) = SINGLE_GAME_INFO.get(&game) {
        song
    } else {
        return Err("Game not found".into());
    };

    add_song_to_spotify_queue(song.clone(), store).await?;
    skip(store).await?;

    store.end_game().await;

    Ok(())
}
