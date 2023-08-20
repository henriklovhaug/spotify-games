use std::sync::OnceLock;

use reqwest::Client;

pub mod game_store;
pub mod middleware;
pub mod routes;
pub mod spotify;
pub mod store;

pub static CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Debug)]
pub enum SpotifyTask {
    AddSong(String),
    Game(Game),
}

#[derive(Debug)]
pub enum Game {
    SixMinutes,
    RatlingBog,
}

#[derive(Clone)]
pub struct ChannelMessage {
    pub channel: String,
    pub message: String,
}

impl ChannelMessage {
    pub fn new(channel: String, message: String) -> ChannelMessage {
        ChannelMessage { channel, message }
    }
}
