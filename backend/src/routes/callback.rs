use axum::extract::Query;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

pub async fn callback(Query(param): Query<Params>) -> StatusCode {
    let _ = param.code;
    StatusCode::OK
}
