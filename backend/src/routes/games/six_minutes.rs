use crate::{
    spotify::{api::skip, types::SpotifyActivity},
    store::Store,
};
use axum::{extract::State, http::StatusCode};

pub async fn skip_handler(State(store): State<Store>) -> StatusCode {
    if store.get_activity().await != SpotifyActivity::Game(crate::spotify::types::Games::SixMinutes)
    {
        return StatusCode::BAD_REQUEST;
    }

    if skip(store).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}
