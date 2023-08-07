use axum::extract::{Query, State};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::spotify::token::login;
use crate::store::Store;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

pub async fn callback(State(store): State<Store>, Query(param): Query<Params>) -> StatusCode {
    let auth_code = param.code;
    let _ = login(store, auth_code).await;
    StatusCode::OK
}
