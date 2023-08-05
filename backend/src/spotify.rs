use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SpotifyLogin {
    pub username: String,
    pub password: String,
}

impl SpotifyLogin {
    pub fn new(username: String, password: String) -> SpotifyLogin {
        SpotifyLogin { username, password }
    }
}
