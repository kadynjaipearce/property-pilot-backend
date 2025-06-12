use axum::{
    extract::{Path, State},
    Json,
};
use tracing::{info, warn, error};
use crate::models::users::ForwardingEmail;
use crate::error::Result;
use crate::AppState;

pub async fn get_user(
    State(state): State<AppState>,

) -> Result<Json<ForwardingEmail>> {
    info!("Getting user 1");
    // Placeholder for user retrieval logic
    let user = ForwardingEmail {
        user_id: "1".to_string(),
        email: "forward+1@mydomain.com".to_string(),
    };
    info!("Found user 1");
    Ok(Json(user))
}

pub async fn get_or_create_forwarding_email(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<ForwardingEmail>> {
    info!("Creating forwarding email for user {}", user_id);
    // Your handler logic here...
    let email = ForwardingEmail {
        user_id: user_id.clone(),
        email: format!("forward+{}@mydomain.com", user_id),
    };
    info!("Created forwarding email for user {}", user_id);
    Ok(Json(email))
}
