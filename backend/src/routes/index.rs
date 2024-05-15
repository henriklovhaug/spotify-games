use askama::Template;

pub async fn index_handler() -> IndexTemplate<'static> {
    IndexTemplate { games: &GAMES }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    games: &'a [GamePlayModal<'a>],
}

pub struct GamePlayModal<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub url: &'a str,
    pub target: &'a str,
}

static GAMES: [GamePlayModal; 5] = [
    GamePlayModal {
        name: "Opus",
        id: "opus",
        url: "/Opus",
        target: "opus-t",
    },
    GamePlayModal {
        name: "Six Minutes",
        id: "six",
        url: "/SixMinutes",
        target: "six-t",
    },
    GamePlayModal {
        name: "Rattling Bog",
        id: "rattling",
        url: "/RattlingBog",
        target: "rattling-t",
    },
    GamePlayModal {
        name: "Thunder",
        id: "thunder",
        url: "/Thunder",
        target: "thunder-t",
    },
    GamePlayModal {
        name: "Palmerna",
        id: "palmerna",
        url: "/Palmerna",
        target: "palmerna-t",
    },
];
