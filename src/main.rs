use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
    extract::Request,
};
use shuttle_runtime::SecretStore;
use tracing::{info, warn};

mod database;
mod error;
mod environments;
mod handlers;
mod models;
mod routes;

use crate::database::database::Database;
use crate::environments::Environments;

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
    (StatusCode::NOT_FOUND, "Not Found")
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Load environment variables
    let env = Environments::from_secrets(&secrets);

    // Initialize database
    let db = Database::new(&env).await?;

    // Build our application with a route
    let app = Router::new()
        .route("/", get(default_route))
        .nest("/users", routes::users::user_routes())
        .with_state(AppState { db, env })
        .fallback(not_found_handler);

    Ok(app.into())
}
