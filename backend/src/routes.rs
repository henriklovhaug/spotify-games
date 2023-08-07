use axum::{middleware, routing::get, Router};

use crate::{middleware::token_check::check_auth_token, store::Store};

use self::{callback::callback, currently_playing::get_currently_playing, index::index_handler};

mod callback;
mod currently_playing;
mod games;
mod index;

pub fn generate_routes(store: Store) -> Router {
    Router::new()
        .route("/currently_playing", get(get_currently_playing))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_auth_token,
        ))
        .route("/callback", get(callback))
        .with_state(store)
        .route("/", get(index_handler))
}
