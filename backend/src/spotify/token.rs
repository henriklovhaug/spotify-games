use std::error::Error;

use reqwest::Client;

use crate::{
    spotify::response_types::LoginResponse,
    store::{Store, Token},
    CLIENT,
};

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const REDIRECT_URI: &str = "http://localhost:3000/callback";

const BASE_URL: &str = "https://api.spotify.com/v1";

pub async fn login(store: Store, code: String) -> Result<(), Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
    ];

    let response = client.post(TOKEN_URL).form(&params).send().await?;

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
