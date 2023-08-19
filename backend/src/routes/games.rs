use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::{spotify::types::Games, store::Store};

pub mod six_minutes;

pub async fn start_game(State(store): State<Store>, Path(game): Path<Games>) -> StatusCode {
    StatusCode::OK
}
