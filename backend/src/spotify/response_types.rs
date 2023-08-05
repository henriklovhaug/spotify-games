use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoginResponse {
    #[serde(rename = "access_token")]
    token: String,
    #[serde(rename = "token_type")]
    _type: String,
    #[serde(rename = "expires_in")]
    expires: u32,
}
