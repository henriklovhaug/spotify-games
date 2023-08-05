pub mod routes;
pub mod spotify;
pub mod store;


#[derive(Debug)]
pub enum SpotifyTask {
    AddSong(String),
    Game(Game)
}

#[derive(Debug)]
pub enum Game {
    SixMinutes,
    RatlingBog,
}
