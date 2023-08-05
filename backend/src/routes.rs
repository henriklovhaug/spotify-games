use axum::Router;

use crate::store::Store;

mod currently_playing;
mod games;

pub fn generate_routes(store: Store) -> Router {
    Router::new().with_state(store)
}
