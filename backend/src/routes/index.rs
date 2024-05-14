use askama::Template;

pub async fn index_handler() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
