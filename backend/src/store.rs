use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Store {
    session_token: Arc<RwLock<Token>>,
    song_queue: Arc<RwLock<Vec<String>>>,
}

/// # Global store for the application
///
/// Wrap every stateful object in an Arc<RwLock<T>> to allow for concurrent access
impl Store {
    pub fn new() -> Store {
        Store {
            session_token: Arc::new(RwLock::new(Token::default())),
            song_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn update_session_token(&self, session_token: String, expires: u32) {
        let mut token = self.session_token.write().await;
        *token = Token::new(session_token, expires);
    }

    pub async fn get_session_token(&self) -> String {
        self.session_token.read().await.token.to_string()
    }

    pub async fn get_song_queue(&self) -> Vec<String> {
        self.song_queue.read().await.to_vec()
    }

    pub async fn add_song_to_queue(&self, song: String) {
        let mut queue = self.song_queue.write().await;
        queue.push(song);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Token {
    token: String,
    expires: u32,
}

impl Token {
    pub fn new(token: String, expires: u32) -> Token {
        Token { token, expires }
    }

    pub fn is_expired(&self) -> bool {
        self.expires < 60
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::new(String::new(), 0)
    }
}
