use std::error::Error;

use axum::{
    extract::{Query, State},
    Json,
};
use reqwest::Response;
use serde::Deserialize;

use crate::{spotify::types::Song, store::Store};

#[derive(Debug, Deserialize)]
pub struct Params {
    search: String,
}

const URL: &str = "https://api.spotify.com/v1/search?type=track&q=";

pub async fn search_song_handler(
    State(store): State<Store>,
    Query(param): Query<Params>,
) -> Json<Vec<Song>> {

    let search_str = format!("{}{}", URL, param.search);
    let songs = search(&search_str).await.unwrap();
    todo!("implement search_song_handler")
}

async fn search(search_str: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    todo!("implement search")
}

fn parse_response(response: Response) -> Result<Vec<Song>, Box<dyn Error>> {
    todo!("implement parse_response")
}
