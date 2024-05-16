use std::{collections::VecDeque, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    OwnedRwLockReadGuard, RwLock, RwLockWriteGuard,
};
use tracing::info;

use crate::{
    routes::queue::SongQueueTemplate,
    spotify::{
        response_types::LoginResponse,
        types::{Games, Song, SpotifyActivity},
    },
};

/// # Global store for the application
///
/// Wrap every stateful object in an Arc<RwLock<T>> to allow for concurrent access
#[derive(Debug, Clone)]
pub struct Store {
    session_token: Arc<RwLock<Option<Token>>>,
    song_queue: Arc<RwLock<VecDeque<Song>>>,
    activity: Arc<RwLock<SpotifyActivity>>,
    tx: Sender<String>,
}

impl Default for Store {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(32);
        Store::new(tx)
    }
}

impl Store {
    pub fn new(tx: Sender<String>) -> Store {
        Store {
            session_token: Arc::new(RwLock::new(None)),
            song_queue: Arc::new(RwLock::new(VecDeque::new())),
            activity: Arc::new(RwLock::new(SpotifyActivity::Music)),
            tx,
        }
    }

    pub async fn set_token_manually(
        &self,
        session_token: String,
        expires: u32,
        refresh_token: String,
    ) {
        let mut token = self.session_token.write().await;
        *token = Some(Token::new(session_token, expires, refresh_token));
    }

    pub async fn set_token(&self, token: Token) {
        let mut session_token = self.session_token.write().await;
        info!("Setting token: {:?}", token);
        *session_token = Some(token);
    }

    pub async fn get_session_token(&self) -> Option<String> {
        self.session_token
            .read()
            .await
            .as_ref()
            .map(|v| v.token.clone())
    }

    pub async fn try_get_session_token(&self) -> Result<String, String> {
        let token = self.get_session_token().await;
        match token {
            Some(v) => Ok(v),
            None => Err("No token found".into()),
        }
    }

    pub async fn get_token_write_guard(self) -> OwnedRwLockReadGuard<Option<Token>> {
        self.session_token.read_owned().await
    }

    pub async fn get_token(&self) -> Option<Token> {
        self.session_token.read().await.to_owned()
    }

    pub async fn try_get_token(&self) -> Result<Token, String> {
        let token = self.get_token().await;
        match token.as_ref() {
            Some(v) => Ok(v.to_owned()),
            None => Err("No token found".into()),
        }
    }

    pub async fn get_song_queue(&self) -> VecDeque<Song> {
        self.song_queue.read().await.to_owned()
    }

    pub async fn get_writable_song_queue(&self) -> RwLockWriteGuard<VecDeque<Song>> {
        self.song_queue.write().await
    }

    pub async fn get_next_song(&self) -> Option<Song> {
        let song = {
            let mut queue = self.song_queue.write().await;
            queue.pop_front()
        };

        self.push_queue_changes().await;

        song
    }

    pub async fn view_next_song(&self) -> Option<Song> {
        let queue = self.song_queue.read().await;
        queue.front().cloned()
    }

    pub async fn add_song_to_queue(&self, song: Song) {
        {
            let mut queue = self.song_queue.write().await;
            queue.push_back(song);
        }
        self.push_queue_changes().await;
    }

    pub async fn add_song_to_queue_front(&self, song: Song) {
        {
            let mut queue = self.song_queue.write().await;
            queue.push_front(song);
        }
        self.push_queue_changes().await;
    }

    pub async fn start_game(&self, game: Games) {
        if self.get_activity().await == SpotifyActivity::Game(game) {
            return;
        }
        let mut activity = self.activity.write().await;
        *activity = SpotifyActivity::Game(game);
    }

    pub async fn end_game(&self) {
        let mut activity = self.activity.write().await;
        *activity = SpotifyActivity::Music;
    }

    pub async fn valid_token(&self) -> bool {
        if let Some(v) = self.session_token.read().await.as_ref() {
            !v.is_expired()
        } else {
            false
        }
    }

    pub async fn get_activity(&self) -> SpotifyActivity {
        self.activity.read().await.to_owned()
    }

    pub fn get_receiver(&self) -> Receiver<String> {
        self.tx.subscribe()
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.tx.to_owned()
    }

    pub async fn push_queue_changes(&self) {
        let value = SongQueueTemplate::new(self.get_song_queue().await);
        let r = self.get_sender().send(value.to_string());
        info!("Pushing queue changes {:?}", r);
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub expires: u32,
    creation_time: DateTime<Utc>,
    refresh_token: String,
}

impl From<LoginResponse> for Token {
    fn from(value: LoginResponse) -> Self {
        Token {
            token: value.token,
            expires: value.expires,
            creation_time: Utc::now(),
            refresh_token: value.refresh_token.expect("Refresh token not found"),
        }
    }
}

impl Token {
    pub fn new(token: String, expires: u32, refresh_token: String) -> Token {
        Token {
            token,
            expires,
            creation_time: Utc::now(),
            refresh_token,
        }
    }

    const MINUTE: i64 = 60;

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let duration = now - self.creation_time;

        duration.num_seconds() + Self::MINUTE > self.expires as i64
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.to_owned()
    }
}
