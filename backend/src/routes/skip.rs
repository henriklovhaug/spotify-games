use axum::extract::State;

use crate::{spotify::api::skip, store::Store};

pub async fn skip_n_tracks_handler(State(store): State<Store>, body: String) {
    let mut n: u8 = body.parse().unwrap(); // Handle error

    if n > 10 {
        n = 10;
    }

    for _ in 0..n {
        if let Err(e) = skip(&store).await {
            println!("Error: {:?}", e);
        }
    }
}
