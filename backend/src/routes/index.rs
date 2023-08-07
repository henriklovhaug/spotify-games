use axum::response::Html;

pub async fn index_handler() -> Html<&'static str> {
    let index = include_str!("../templates/index.html");
    Html(index)
}
