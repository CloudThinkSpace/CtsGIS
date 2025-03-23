use axum::{Router, middleware};
use cts_middleware::middleware::auth::auth_middleware;
pub mod aggregate;
pub mod feature;
pub mod mbtiles;
pub mod search;
pub mod user;

pub fn auth_router() -> Router {
    Router::new()
        .nest("/feature", feature::feature_router())
        .nest("/mbtiles", mbtiles::mbtiles_router())
        .nest("/search", search::search_router())
        .nest("/aggregate", aggregate::aggregate_router())
        .nest("/user", user::user_router())
        .layer(middleware::from_fn(auth_middleware))
}
