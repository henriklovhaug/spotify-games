use crate::store::Store;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn skip(State(store): State<Store>) -> StatusCode {
    StatusCode::OK
}
