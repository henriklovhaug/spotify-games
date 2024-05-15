use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{spotify::token::refresh_token, store::Store};

pub async fn check_token_lifetime(
    State(store): State<Store>,
    request: Request,
    next: Next,
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
