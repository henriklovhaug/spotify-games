use std::sync::OnceLock;

use reqwest::Client;

pub mod game_store;
pub mod middleware;
pub mod routes;
pub mod spotify;
pub mod store;

pub static CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub channel: String,
    pub message: String,
}

impl ChannelMessage {
    pub fn new(channel: String, message: String) -> ChannelMessage {
        ChannelMessage { channel, message }
    }
}
