use dotenvy::dotenv;
use tokio::sync::broadcast;
use std::net::SocketAddr;

use backend::{
    routes::generate_routes,
    spotify::{event_loop::spotify_loop, token::restore_token_from_file},
    store::Store,
    ChannelMessage,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (tx, mut rx) = broadcast::channel::<ChannelMessage>(32);
    let store = Store::new(tx, rx);

    if let Err(e) = restore_token_from_file(store.clone()).await {
        println!(
            "Failed to restore token from file: {}\n continues server",
            e
        );
    }

    let store_clone = store.clone();
    tokio::spawn(async move {
        spotify_loop(store_clone).await;
    });

    let routes = generate_routes(store);

    let addr = "[::]:4000".parse::<SocketAddr>().unwrap();

    axum::Server::bind(&addr)
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
