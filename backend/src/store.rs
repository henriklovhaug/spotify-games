use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::{OwnedRwLockReadGuard, RwLock, RwLockReadGuard};

use crate::{
    spotify::{
        response_types::LoginResponse,
        types::{Song, SpotifyActivity},
    },
    SpotifyTask,
};

#[derive(Debug, Clone)]
pub struct Store {
    session_token: Arc<RwLock<Option<Token>>>,
    song_queue: Arc<RwLock<Vec<Song>>>,
    tasks: Arc<RwLock<Vec<SpotifyTask>>>,
    activity: Arc<RwLock<SpotifyActivity>>,
}

/// # Global store for the application
///
/// Wrap every stateful object in an Arc<RwLock<T>> to allow for concurrent access
impl Store {
    pub fn new() -> Store {
        Store {
            session_token: Arc::new(RwLock::new(None)),
            song_queue: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(RwLock::new(Vec::new())),
            activity: Arc::new(RwLock::new(SpotifyActivity::Music)),
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
        println!("Setting token: {:?}", token);
        *session_token = Some(token);
    }

    pub async fn get_session_token(&self) -> Option<String> {
        self.session_token
            .read()
            .await
            .as_ref()
            .map(|v| v.token.clone())
    }

    pub async fn get_token_owned(self) -> OwnedRwLockReadGuard<Option<Token>> {
        self.session_token.read_owned().await
    }

    pub async fn get_token(&self) -> RwLockReadGuard<Option<Token>> {
        self.session_token.read().await
    }

    pub async fn add_task(&self, task: SpotifyTask) {
        let mut tasks = self.tasks.write().await;
        tasks.push(task);
    }

    pub async fn get_song_queue(&self) -> Vec<Song> {
        self.song_queue.read().await.to_vec()
    }

    pub async fn add_song_to_queue(&self, song: Song) {
        let mut queue = self.song_queue.write().await;
        queue.push(song);
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
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
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
