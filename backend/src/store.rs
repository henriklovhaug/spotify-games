use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::{spotify::response_types::LoginResponse, SpotifyTask};

#[derive(Debug, Clone)]
pub struct Store {
    session_token: Arc<RwLock<Token>>,
    song_queue: Arc<RwLock<Vec<String>>>,
    tasks: Arc<RwLock<Vec<SpotifyTask>>>,
}

/// # Global store for the application
///
/// Wrap every stateful object in an Arc<RwLock<T>> to allow for concurrent access
impl Store {
    pub fn new() -> Store {
        Store {
            session_token: Arc::new(RwLock::new(Token::default())),
            song_queue: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn set_token_manually(&self, session_token: String, expires: u32) {
        let mut token = self.session_token.write().await;
        *token = Token::new(session_token, expires);
    }

    pub async fn set_token(&self, token: Token) {
        let mut session_token = self.session_token.write().await;
        println!("Setting token: {:?}", token);
        *session_token = token;
    }

    pub async fn get_session_token(&self) -> String {
        self.session_token.read().await.token.to_owned()
    }

    pub async fn get_song_queue(&self) -> Vec<String> {
        self.song_queue.read().await.to_vec()
    }

    pub async fn add_song_to_queue(&self, song: String) {
        let mut queue = self.song_queue.write().await;
        queue.push(song);
    }

    pub async fn valid_token(&self) -> bool {
        !self.session_token.read().await.is_expired()
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
}

impl From<LoginResponse> for Token {
    fn from(value: LoginResponse) -> Self {
        Token {
            token: value.token,
            expires: value.expires,
            creation_time: Utc::now(),
        }
    }
}

impl Token {
    pub fn new(token: String, expires: u32) -> Token {
        Token {
            token,
            expires,
            creation_time: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let duration = now - self.creation_time;

        duration.num_seconds() > self.expires as i64
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::new(String::new(), 0)
    }
}
