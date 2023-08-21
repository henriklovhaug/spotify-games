use axum::{extract::State, Json};

use crate::{
    spotify::{api::get_current_song, types::CurrentSong},
    store::Store,
};

pub async fn get_currently_playing_handler(
    State(store): State<Store>,
) -> Result<Json<CurrentSong>, String> {
    let song = get_current_song(&store).await.map_err(|e| e.to_string())?;

    Ok(Json(song))
}
