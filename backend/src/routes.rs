use axum::{routing::get, Router};

use crate::store::Store;

use self::currently_playing::get_currently_playing;

mod currently_playing;
mod games;

pub fn generate_routes(store: Store) -> Router {
    Router::new()
        .route("/currently_playing", get(get_currently_playing))
        .with_state(store)
}
