use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};
use shuttle_runtime::SecretStore;

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
    "Welcome to Property Pilot API"
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
        .with_state(AppState { db, env });

    Ok(app.into())
}
