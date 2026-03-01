use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use tracing::{info, warn};

use crate::{error::ApiError, state::AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    info!(
        service = state.service_name,
        version = state.service_version,
        "health endpoint called"
    );

    Json(HealthResponse {
        status: "ok",
        service: state.service_name,
        version: state.service_version,
    })
}

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<HelloResponse>, ApiError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        warn!("hello endpoint received an empty name parameter");
        return Err(ApiError::bad_request("name path parameter cannot be empty"));
    }

    info!(name = %trimmed, "hello endpoint called");

    Ok(Json(HelloResponse {
        message: format!(
            "Hello, {trimmed}! Welcome to {} v{}.",
            state.service_name, state.service_version
        ),
    }))
}
