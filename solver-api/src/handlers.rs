use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use crate::solver::{spawn_solve, JobStore};
use crate::types::{JobState, JobSubmission, SolveRequest};

#[derive(Clone)]
pub struct AppState {
    pub jobs: JobStore,
}

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

/// AGPL-3.0 section 13 source-offer endpoint. This service links against
/// postflop-solver (AGPL-3.0), so anyone interacting with it over the
/// network is entitled to the complete corresponding source of the exact
/// version running here, including any local modifications. This endpoint
/// is the discoverable, machine-readable way to satisfy that obligation —
/// see COMPLIANCE.md and NOTICE for the full explanation.
///
/// Operators: if you deploy a modified build, set SOLVER_SOURCE_URL to the
/// public repository (or source archive) URL for YOUR running version. The
/// default below only satisfies AGPL for the unmodified upstream project.
pub async fn source() -> impl IntoResponse {
    let url = std::env::var("SOLVER_SOURCE_URL").unwrap_or_else(|_| {
        "https://github.com/josephhiggins-boss/PokerTracker-Solver".to_string()
    });
    Json(json!({
        "license": "AGPL-3.0-or-later",
        "source_url": url,
        "notice": "This service wraps postflop-solver (https://github.com/b-inary/postflop-solver, AGPL-3.0) and is itself licensed AGPL-3.0. Under section 13, you are entitled to the complete corresponding source code of the exact version of this service you are interacting with, including any modifications, at no charge. See source_url. If source_url does not reflect the version currently running, that is a compliance bug in this deployment \u{2014} please report it."
    }))
}

pub async fn post_solve(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SolveRequest>,
) -> impl IntoResponse {
    let job_id = Uuid::new_v4();
    info!(%job_id, board = %req.board, "accepted solve");
    spawn_solve(state.jobs.clone(), job_id, req);
    (StatusCode::ACCEPTED, Json(JobSubmission { job_id }))
}

pub async fn get_solve(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<JobState>, (StatusCode, Json<serde_json::Value>)> {
    let Some(entry) = state.jobs.get(&id) else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "job not found" })),
        ));
    };
    let guard = entry.read().await;
    Ok(Json(guard.clone()))
}
