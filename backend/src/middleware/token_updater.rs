use axum::{extract::State, http::Request, middleware::Next, response::Response};

use crate::{spotify::token::refresh_token, store::Store};

pub async fn check_token_lifetime<T>(
    State(store): State<Store>,
    request: Request<T>,
    next: Next<T>,
) -> Response {
    if !store.valid_token().await {
        if let Some(token) = store.get_token().await.as_ref() {
            if let Err(e) = refresh_token(&store, &token.refresh_token()).await {
                println!("Error refreshing token: {}", e);
            }
        };
    }

    next.run(request).await
}
