use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::{spotify::response_types::LoginResponse, SpotifyTask};

#[derive(Debug, Clone)]
pub struct Store {
    session_token: Arc<RwLock<Option<Token>>>,
    song_queue: Arc<RwLock<Vec<String>>>,
    tasks: Arc<RwLock<Vec<SpotifyTask>>>,
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
        if let Some(v) = self.session_token.read().await.as_ref() {
            Some(v.token.clone())
        } else {
            None
        }
    }

    pub async fn add_task(&self, task: SpotifyTask) {
        let mut tasks = self.tasks.write().await;
        tasks.push(task);
    }

    pub async fn get_song_queue(&self) -> Vec<String> {
        self.song_queue.read().await.to_vec()
    }

    pub async fn add_song_to_queue(&self, song: String) {
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
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
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
            refresh_token: value.refresh_token,
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

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let duration = now - self.creation_time;

        duration.num_seconds() > self.expires as i64
    }
}
