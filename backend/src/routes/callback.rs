use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;

use crate::spotify::token::{login, save_refresh_token};
use crate::store::Store;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

pub async fn callback_handler(
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
