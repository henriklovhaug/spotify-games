use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    headers::UserAgent,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
    Router, TypedHeader,
};

use crate::{
    middleware::{token_check::check_auth_token, token_updater::check_token_lifetime},
    store::Store,
};
use futures::{sink::SinkExt, stream::StreamExt};

use self::{
    callback::callback_handler,
    currently_playing::get_currently_playing_handler,
    games::{six_minutes::skip_handler, start_game},
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
        .nest("/sixminutes", six_minutes_routes(store))
}

fn six_minutes_routes(store: Store) -> Router {
    Router::new()
        .route("/skip", put(skip_handler))
        .with_state(store)
}

async fn ws_handler(
    State(store): State<Store>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, store))
}
async fn handle_socket(mut socket: WebSocket, who: SocketAddr, store: Store) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients from
    // connecting to server and receiving their greetings.
    // for i in 1..5 {
    //     if socket
    //         .send(Message::Text(format!("Hi {i} times!")))
    //         .await
    //         .is_err()
    //     {
    //         println!("client {who} abruptly disconnected");
    //         return;
    //     }
    //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    // }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut rx = store.get_receiver();

        while let Ok(message) = rx.recv().await {
            if sender
                .send(Message::Text(format!(
                    "Server message {} ...",
                    message.message
                )))
                .await
                .is_err()
            {
                return "kek";
            }
        }
        "kek"
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            println!("Received message: {:?}", msg);
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{} messages sent to {}", a, who),
                Err(a) => println!("Error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {} messages", b),
                Err(b) => println!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {} destroyed", who);
}
