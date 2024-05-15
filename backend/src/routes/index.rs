use askama::Template;

pub async fn index_handler() -> IndexTemplate<'static> {
    IndexTemplate { search: "" }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    search: &'a str,
}
