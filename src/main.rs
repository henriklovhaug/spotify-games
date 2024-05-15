use dotenvy::dotenv;
use listenfd::ListenFd;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use backend::{
    routes::generate_routes,
    spotify::{event_loop::spotify_loop, token::restore_token_from_file},
    store::Store,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let store = Store::default();

    if let Err(e) = restore_token_from_file(&store).await {
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind(&addr).await.unwrap(),
    };

    info!("Starting server at: {}", addr);
    axum::serve(
        listener,
        routes.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
