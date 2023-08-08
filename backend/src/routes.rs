use axum::{
    middleware,
    routing::{get, put},
    Router,
};

use crate::{middleware::token_check::check_auth_token, store::Store};

use self::{
    callback::callback, currently_playing::get_currently_playing, index::index_handler,
    pause::pause_music, skip::skip_n_tracks,
};

mod callback;
mod currently_playing;
mod games;
mod index;
mod pause;
mod skip;

pub fn generate_routes(store: Store) -> Router {
    Router::new()
        .route("/currently_playing", get(get_currently_playing))
        .route("/pause", put(pause_music))
        .route("/skip", put(skip_n_tracks))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_auth_token,
        ))
        .route("/callback", get(callback))
        .with_state(store)
        .route("/", get(index_handler))
}
