use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    id: String,
    name: String,
    artist: String,
    album: String,
    duration: u32,
    is_playing: bool,
}

impl Song {
    pub fn new(
        id: String,
        name: String,
        artist: String,
        album: String,
        duration: u32,
        is_playing: bool,
    ) -> Song {
        Song {
            id,
            name,
            artist,
            album,
            duration,
            is_playing,
        }
    }
}
