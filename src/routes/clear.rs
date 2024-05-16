use crate::store::Store;
use axum::extract::State;
use axum::response::Redirect;

pub async fn clear_queue_handler(State(store): State<Store>) -> Redirect {
    store.get_writable_song_queue().await.clear();
    Redirect::to("/")
}
