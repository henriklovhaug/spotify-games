use axum::{routing::get, Router};

use crate::store::Store;

use self::{callback::callback, currently_playing::get_currently_playing};

mod callback;
mod currently_playing;
mod games;

pub fn generate_routes(store: Store) -> Router {
    Router::new()
        .route("/currently_playing", get(get_currently_playing))
        .route("/callback", get(callback))
        .with_state(store)
}
