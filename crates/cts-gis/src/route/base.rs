use axum::{Router, routing::get};
use login::login_router;

pub mod login;

pub fn noauth_router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/base", login_router())
}

async fn root() -> &'static str {
    "Welcome to CTS GIS Server!"
}
