use crate::store::Store;

pub async fn spotify_loop(store: Store) -> () {
    loop {
        store.get_song_queue().await;
        unimplemented!();
    }
}
