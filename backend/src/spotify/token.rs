use std::{env, error::Error};

use base64::{engine::general_purpose, Engine};
use reqwest::Client;

use crate::{
    spotify::response_types::LoginResponse,
    store::{Store, Token},
    CLIENT,
};

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const REDIRECT_URI: &str = "http://localhost:3000/callback";

pub async fn login(store: Store, code: String) -> Result<(), Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
    ];

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    let client_id_secret = format!("{}:{}", client_id, client_secret);
    let base64_auth = general_purpose::STANDARD.encode(client_id_secret.as_bytes());

    let response = client
        .post(TOKEN_URL)
        .header("Authorization", format!("Basic {}", base64_auth))
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Log in successfull");
    } else {
        println!("Log in failed");
        println!("{:?}", response);
        return Err("Log in failed".into());
    }

    let token: Token = response.json::<LoginResponse>().await?.into();

    store.set_token(token).await;

    Ok(())
}
