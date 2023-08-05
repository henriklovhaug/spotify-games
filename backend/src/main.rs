use dotenvy::dotenv;
use std::net::SocketAddr;

use backend::{routes::generate_routes, store::Store};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();

    let store = Store::default();

    let routes = generate_routes(store);

    let addr = "[::]:3000".parse::<SocketAddr>().unwrap();

    axum::Server::bind(&addr)
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
