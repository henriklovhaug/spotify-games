use std::{env, error::Error};

use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    spotify::response_types::LoginResponse,
    store::{Store, Token},
    CLIENT,
};

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const REDIRECT_URI: &str = "http://localhost:4000/callback";

pub async fn login(store: Store, code: String) -> Result<(), Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
    ];

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    let base64_auth = base64_encode(&client_id, &client_secret);

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

pub async fn refresh_token(store: Store, refresh_token: &str) -> Result<(), Box<dyn Error>> {
    let client = CLIENT.get_or_init(Client::new);

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ];

    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    let base64_auth = base64_encode(&client_id, &client_secret);

    let response = client
        .post(TOKEN_URL)
        .header("Authorization", format!("Basic {}", base64_auth))
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Refresh token successfull");
    } else {
        println!("Refresh token failed");
        println!("{:?}", response);
        return Err("Refresh token failed".into());
    }

    let mut parsed_response: LoginResponse = response.json::<LoginResponse>().await?;

    if !parsed_response.refresh_exists() {
        parsed_response.set_refresh_token(refresh_token.to_string());
    }

    store.set_token(parsed_response.into()).await;
    save_refresh_token(store).await?;

    Ok(())
}

fn base64_encode(id: &str, secret: &str) -> String {
    let id_secret = format!("{}:{}", id, secret);
    let base64_auth = general_purpose::STANDARD.encode(id_secret.as_bytes());
    base64_auth
}

pub async fn restore_token_from_file(store: Store) -> Result<(), Box<dyn Error>> {
    let path = dirs::data_dir()
        .ok_or::<String>("File not found".into())?
        .join("spotify-game")
        .join("data.txt");

    let mut file = File::open(path).await?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    println!("Refresh token: {}", contents);

    refresh_token(store, &contents).await
}

pub async fn save_refresh_token(store: Store) -> Result<(), Box<dyn Error>> {
    let refresh_token = if let Some(v) = store.get_token().await.as_ref() {
        v.refresh_token()
    } else {
        return Err("No session token found".into());
    };

    create_dir();

    let path = dirs::data_dir()
        .expect("No data directory found")
        .join("spotify-game");

    let mut file = File::create(format!(
        "{}/data.txt",
        path.to_str().expect("Path does not exists")
    ))
    .await?;

    // TODO: Encrypt refresh token
    file.write_all(refresh_token.as_bytes()).await?;

    Ok(())
}

fn create_dir() {
    let data_dir = dirs::data_dir().expect("No home directory found");
    let path = data_dir.join("spotify-game");
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Failed to create config directory");
    }
}
