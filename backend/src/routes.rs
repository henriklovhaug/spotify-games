use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{
    middleware::{token_check::check_auth_token, token_updater::check_token_lifetime},
    store::Store,
};

use self::{
    callback::callback_handler,
    currently_playing::get_currently_playing_handler,
    index::index_handler,
    pause::pause_music_handler,
    queue::{add_to_queue_handler, get_queue_handler},
    search::search_song_handler,
    skip::skip_n_tracks_handler,
};

mod callback;
mod currently_playing;
mod games;
mod index;
mod pause;
mod queue;
mod search;
mod skip;

pub fn generate_routes(store: Store) -> Router {
    Router::new()
        .route("/currently_playing", get(get_currently_playing_handler))
        .route("/pause", put(pause_music_handler))
        .route("/skip", put(skip_n_tracks_handler))
        .route("/queue", post(add_to_queue_handler))
        .route("/queue", get(get_queue_handler))
        .route("/search", get(search_song_handler))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_token_lifetime,
        ))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_auth_token,
        ))
        .route("/callback", get(callback_handler))
        .with_state(store)
        .route("/", get(index_handler))
}
