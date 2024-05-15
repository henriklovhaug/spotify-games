use askama::Template;

pub async fn index_handler() -> IndexTemplate<'static> {
    IndexTemplate { games: &GAMES }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    games: &'a [GamePlayStruct<'a>],
}

pub struct GamePlayStruct<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub url: &'a str,
    pub target: &'a str,
}

static GAMES: [GamePlayStruct; 2] = [
    GamePlayStruct {
        name: "Opus",
        id: "opus",
        url: "/Opus",
        target: "opus-t",
    },
    GamePlayStruct {
        name: "Six Minutes",
        id: "six",
        url: "/SixMinutes",
        target: "six-t",
    },
];
