use serde::Deserialize;

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
    pub refresh_token: String,
}
