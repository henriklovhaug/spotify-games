use std::collections::HashMap;

use super::types::{Games, Song};

use lazy_static::lazy_static;

pub mod opus;
pub mod palmerna;
pub mod rattling_bog;
pub mod six_minutes;

lazy_static! {
    pub static ref SINGLE_GAME_INFO: HashMap<Games, Song> = {
        let mut m = HashMap::new();
        m.insert(
            Games::Palmerna,
            Song::new(
                "5hsZ6loP0rseyjleWs0cZ1",
                "Der Palmane Bor",
                "Medena",
                "",
                225226,
                Some("https://i.scdn.co/image/ab67616d0000b2732a73b3592817536ffa7217c9"),
            ),
        );
        m.insert(
            Games::Opus,
            Song::new(
                "3v2oAQomhOcYCPPHafS3KV",
                "Opus",
                "Eric Prydz",
                "",
                543453,
                Some("https://i.scdn.co/image/ab67616d0000b27324492f2ba3a1d995e1faf5d8"),
            ),
        );

        m.insert(
            Games::RattlingBog,
            Song::new(
                "2vjrfvthqjw7bs8as4vdzi",
                "rattling bog",
                "carlyle fraser",
                "lord of the dance",
                142306,
                Some("https://i.scdn.co/image/ab67616d0000b273e973781d62a8142984e624d6"),
            ),
        );

        m
    };
}
