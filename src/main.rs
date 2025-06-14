use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
    extract::Request,
    middleware,
    Json,
};
use shuttle_runtime::SecretStore;
use tracing::{info, warn};
use serde_json::Value;
use crate::error::ApiResponse;

mod auth;
mod database;
mod error;
mod environments;
mod handlers;
mod models;
mod routes;

use crate::database::database::Database;
use crate::environments::Environments;
use crate::auth::middleware::auth_middleware;

#[derive(Clone)]
pub struct AppState {
    db: Database,
    env: Environments,
}

async fn default_route() -> impl IntoResponse {
    info!("Received request to /");
    "Welcome to Property Pilot API"
}

// Fallback handler for 404s
async fn not_found_handler(req: Request) -> impl IntoResponse {
    warn!("Route not found: {} {}", req.method(), req.uri().path());
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::<Value> {
            message: format!("Route not found: {} {}", req.method(), req.uri().path()),
            error: Some("NOT_FOUND".to_string()),
            body: None,
        })
    )
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Load environment variables
    let env = Environments::from_secrets(&secrets);

    // Initialize database
    let db = Database::new(&env).await?;

    // Create app state
    let state = AppState { db, env };

    // Build our application with a route
    let app = Router::new()
        .route("/", get(default_route))
        .nest("/users", routes::users::user_routes())
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
        .fallback(not_found_handler);

    Ok(app.into())
}
