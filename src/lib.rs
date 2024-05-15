extern crate lazy_static;

use std::sync::OnceLock;

use reqwest::Client;
use serde::Serialize;

pub mod middleware;
pub mod routes;
pub mod spotify;
pub mod store;
pub mod ws;

pub static CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
pub struct ChannelMessage {
    pub channel: Channel,
    pub message: String,
    pub artist: Option<String>,
    pub song: Option<String>,
}

impl ChannelMessage {
    pub fn new(
        channel: Channel,
        message: String,
        artist: Option<String>,
        song: Option<String>,
    ) -> ChannelMessage {
        ChannelMessage {
            channel,
            message,
            artist,
            song,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Channel {
    QueueSong,
    SixMinutes,
    QueueList
}
