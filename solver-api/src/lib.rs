pub mod handlers;
pub mod solver;
pub mod types;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use dashmap::DashMap;

use crate::handlers::{get_solve, health, post_solve, source, AppState};

pub fn new_app_state() -> Arc<AppState> {
    Arc::new(AppState {
        jobs: Arc::new(DashMap::new()),
    })
}

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/source", get(source))
        .route("/solve", post(post_solve))
        .route("/solve/:id", get(get_solve))
        .with_state(state)
}
