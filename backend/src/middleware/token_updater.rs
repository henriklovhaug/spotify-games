use axum::{extract::State, http::Request, middleware::Next, response::Response};

use crate::{spotify::token::refresh_token, store::Store};

pub async fn check_token_lifetime<T>(
    State(store): State<Store>,
    request: Request<T>,
    next: Next<T>,
) -> Response {
    if !store.valid_token().await {
        if let Some(token) = store.get_token().await.as_ref() {
            let _ = refresh_token(store.clone(), &token.refresh_token()).await;
        };
    }

    next.run(request).await
}
