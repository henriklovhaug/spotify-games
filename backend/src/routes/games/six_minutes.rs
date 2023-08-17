use crate::store::Store;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn play_six_minutes_handler(State(store): State<Store>) -> StatusCode {
    StatusCode::OK
}
