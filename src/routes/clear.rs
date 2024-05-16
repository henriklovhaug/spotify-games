use axum::extract::State;
use axum::response::Redirect;
use crate::store::Store;


pub async fn clear_queue_handler(State(store): State<Store>) -> Redirect {
    store.get_writable_song_queue().await.clear();
    Redirect::to("/")
}
