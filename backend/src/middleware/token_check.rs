use std::env;

use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::{Redirect, Response},
};
use urlencoding::encode;

use crate::store::Store;

const BASE_URL: &str = "https://accounts.spotify.com/authorize?";
const REDIRECT_URI: &str = "http://localhost:3000/callback";
const SCOPE: &str = "user-read-private user-read-email user-read-playback-state";

pub async fn check_auth_token<T>(
    State(store): State<Store>,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, Redirect> {
    let token = store.get_session_token().await;
    if token.is_some() {
        Ok(next.run(request).await)
    } else {
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");

        let scopes = encode(SCOPE);

        let url = format!(
            "{}client_id={}&response_type=code&redirect_uri={}&scope={}",
            BASE_URL, client_id, REDIRECT_URI, scopes
        );

        Err(Redirect::to(&url))
    }
}
