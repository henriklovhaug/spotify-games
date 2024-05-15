use axum::extract::State;
use axum::http::StatusCode;
use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH};
use reqwest::Client;
use tracing::error;

use crate::store::Store;
use crate::CLIENT;

const URL: &str = "https://api.spotify.com/v1/me/player/pause";

pub async fn pause_music_handler(State(store): State<Store>) -> StatusCode {
    let client = CLIENT.get_or_init(Client::new);

    let token = store.get_session_token().await;

    let response = client
        .put(URL)
        .header(AUTHORIZATION, format!("Bearer {}", token.unwrap()))
        .header(CONTENT_LENGTH, 0)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        let body = response.text().await.unwrap();
        error!("Pause music failed {:?}", body);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    response.status()
}
