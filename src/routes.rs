use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, TraceLayer},
};

use crate::{
    middleware::{token_check::check_auth_token, token_updater::check_token_lifetime},
    store::Store,
    ws::ws_handler,
};

use self::{
    callback::callback_handler,
    currently_playing::get_currently_playing_handler,
    games::{get_game_art, six_minutes::skip_handler, start_game, stop_game},
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
        .route("/game/:game", put(start_game))
        .route("/game/stop", put(stop_game))
        .route("/game/:game", get(get_game_art))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_token_lifetime,
        ))
        .layer(middleware::from_fn_with_state(
            store.clone(),
            check_auth_token,
        ))
        .route("/callback", get(callback_handler))
        .route("/ws", get(ws_handler))
        .with_state(store.clone())
        .route("/", get(index_handler))
        .route_service("/favicon.ico", ServeFile::new("style/favicon.ico"))
        .nest("/sixminutes", six_minutes_routes(store))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
}

fn six_minutes_routes(store: Store) -> Router {
    Router::new()
        .route("/skip", put(skip_handler))
        .with_state(store)
}
