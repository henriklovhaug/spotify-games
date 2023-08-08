use std::error::Error;

use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::spotify::token::login;
use crate::store::Store;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

pub async fn callback(
    State(store): State<Store>,
    Query(param): Query<Params>,
) -> Result<Redirect, String> {
    let auth_code = param.code;
    if let Err(err) = login(store.clone(), auth_code).await {
        return Err(format!("Log in failed: {}", err));
    }
    save_refresh_token(store)
        .await
        .map_err(|err| format!("Failed to save refresh token: {}", err))?;
    Ok(Redirect::to("/"))
}

async fn save_refresh_token(store: Store) -> Result<(), Box<dyn Error>> {
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
