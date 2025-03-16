use crate::config::Config;
use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app: &AppState) -> Self {
        app.pool.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AppState1 {
    pub name: String,
}
