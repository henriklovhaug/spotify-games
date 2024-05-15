use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration: u32,
    pub album_url: Option<String>,
}

impl Song {
    pub fn new<T: ToString>(
        id: T,
        name: T,
        artist: T,
        album: T,
        duration: u32,
        album_url: Option<T>,
    ) -> Song {
        Song {
            id: id.to_string(),
            name: name.to_string(),
            artist: artist.to_string(),
            album: album.to_string(),
            duration,
            album_url: album_url.map(|s| s.to_string()),
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
    album_url: Option<String>,
}

impl CurrentSong {
    pub fn new(
        id: String,
        name: String,
        artist: String,
        album: String,
        duration: u32,
        progress: u32,
        album_url: Option<String>,
    ) -> CurrentSong {
        CurrentSong {
            id,
            name,
            artist,
            album,
            duration,
            progress,
            album_url,
        }
    }

    pub fn get_remaining_time(&self) -> Duration {
        Duration::seconds((self.duration - self.progress) as i64 / 1000)
    }

    pub fn get_artist(&self) -> &str {
        &self.artist
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<CurrentSong> for Song {
    fn from(val: CurrentSong) -> Self {
        Song::new(val.id, val.name, val.artist, val.album, val.duration, None)
    }
}

#[derive(Debug, Serialize)]
pub struct QueueSong {
    name: String,
    artist: String,
}

impl QueueSong {
    pub fn new(name: String, artist: String) -> QueueSong {
        QueueSong { name, artist }
    }

    pub fn get_artist(&self) -> &str {
        &self.artist
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<Song> for QueueSong {
    fn from(val: Song) -> Self {
        QueueSong::new(val.name, val.artist)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpotifyActivity {
    Music,
    Game(Games),
}

#[derive(Debug, Hash, Eq, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Games {
    SixMinutes,
    RattlingBog,
    Opus,
    Palmerna,
    Thunder,
}
