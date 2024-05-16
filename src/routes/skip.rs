use axum::extract::State;
use tracing::error;

use crate::{
    spotify::api::{add_song_to_spotify_queue, skip},
    store::Store,
};

pub async fn skip_n_tracks_handler(State(store): State<Store>) {
    if store.view_next_song().await.is_some() {
        let next_song = store.get_next_song().await.unwrap();
        if let Err(e) = add_song_to_spotify_queue(next_song, &store).await {
            error!("Error adding song to queue: {}", e);
        }
    }

    if let Err(e) = skip(&store).await {
        error!("Error skipping track: {:?}", e);
    }
}
