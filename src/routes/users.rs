use axum::{routing::{get, post}, Router};
use crate::handlers::users;
use crate::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(users::get_user))
        .route("/{user_id}/forwarding-address", post(users::get_or_create_forwarding_email))
} 