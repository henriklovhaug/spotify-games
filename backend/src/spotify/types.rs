use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    id: String,
    name: String,
    artist: String,
    album: String,
    duration: u32,
}

impl Song {
    pub fn new(id: String, name: String, artist: String, album: String, duration: u32) -> Song {
        Song {
            id,
            name,
            artist,
            album,
            duration,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Serialize)]
pub struct CurrentSong {
    id: String,
    name: String,
    artist: String,
    album: String,
    duration: u32,
    progress: u32,
}

impl CurrentSong {
    pub fn new(
        id: String,
        name: String,
        artist: String,
        album: String,
        duration: u32,
        progress: u32,
    ) -> CurrentSong {
        CurrentSong {
            id,
            name,
            artist,
            album,
            duration,
            progress,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpotifyActivity {
    Music,
    Game,
}
