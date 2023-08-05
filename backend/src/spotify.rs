use std::env;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SpotifyLogin {
    pub client_id: String,
    pub client_secret: String,
}

impl SpotifyLogin {
    pub fn new(client_id: String, client_secret: String) -> SpotifyLogin {
        SpotifyLogin {
            client_id,
            client_secret,
        }
    }
}

impl Default for SpotifyLogin {
    fn default() -> Self {
        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
        let client_secret =
            env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

        SpotifyLogin {
            client_id,
            client_secret,
        }
    }
}
