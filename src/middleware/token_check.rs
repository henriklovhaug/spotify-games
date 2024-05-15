use std::env;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{Redirect, Response},
};
use urlencoding::encode;

use crate::store::Store;

const BASE_URL: &str = "https://accounts.spotify.com/authorize?";
const SCOPE: &str =
    "user-read-private user-read-email user-read-playback-state user-modify-playback-state";

pub async fn check_auth_token(
    State(store): State<Store>,
    request: Request,
    next: Next,
) -> Result<Response, Redirect> {
    let redirect_uri = env::var("REDIRECT_URL").expect("REDIRECT_URL not set");

    let token = store.get_session_token().await;
    if token.is_some() {
        Ok(next.run(request).await)
    } else {
        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
        let scopes = encode(SCOPE);
        let redirect_uri = encode(&redirect_uri);

        let url = format!(
            "{}client_id={}&response_type=code&redirect_uri={}&scope={}",
            BASE_URL, client_id, redirect_uri, scopes
        );

        Err(Redirect::to(&url))
    }
}
