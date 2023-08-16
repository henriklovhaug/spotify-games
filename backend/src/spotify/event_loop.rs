use crate::store::Store;

pub async fn spotify_loop(store: Store) {
    loop {
        let queue = store.get_song_queue().await;
        if queue.is_empty() {
            continue;
        }
        unimplemented!();
    }
}

fn get_current_song() {
    unimplemented!();
}
