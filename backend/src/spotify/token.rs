use std::error::Error;

use reqwest::Client;

use crate::{store::Store, CLIENT};

use super::{response_types::LoginResponse, SpotifyLogin};

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub async fn login(store: Store) -> Result<(), Box<dyn Error>> {
    let login = SpotifyLogin::default();

    let client = CLIENT.get_or_init(Client::new);

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", login.client_id.as_str()),
        ("client_secret", login.client_secret.as_str()),
    ];

    let response = client.post(TOKEN_URL).form(&params).send().await?;

    if response.status().is_success() {
        println!("Log in successfull");
    } else {
        println!("Log in failed");
        println!("{:?}", response);
        return Err("Log in failed".into());
    }

    store
        .set_token(Into::into(response.json::<LoginResponse>().await?))
        .await;

    Ok(())
}
