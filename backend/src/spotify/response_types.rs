use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "access_token")]
    pub token: String,
    #[serde(rename = "token_type")]
    _type: String,
    #[serde(rename = "expires_in")]
    pub expires: u32,
    #[serde(rename = "scope")]
    _scope: String,
    pub refresh_token: Option<String>,
}

impl LoginResponse {
    pub fn refresh_exists(&self) -> bool {
        self.refresh_token.is_some()
    }

    pub fn set_refresh_token(&mut self, refresh_token: String) {
        info!("Setting refresh token");
        self.refresh_token = Some(refresh_token);
    }
}
