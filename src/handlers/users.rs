use axum::{
    extract::{Path, State},
    Json,
};
use tracing::{info, warn, error};
use crate::models::users::ForwardingEmail;
use crate::error::Result;
use crate::AppState;
use crate::auth::types::AuthUser;


pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    user: AuthUser,
) -> Result<Json<ForwardingEmail>> {
    info!("Getting user {} (authenticated as {})", user_id, user.id);
    
    // Your handler logic here...
    let email = ForwardingEmail {
        user_id: user_id.clone(),
        email: format!("forward+{}@mydomain.com", user_id),
    };
    
    Ok(Json(email))
}


pub async fn create_user(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ForwardingEmail>> {
    info!("Creating user (authenticated as {})", user.id);
    
    // Your handler logic here...
    let email = ForwardingEmail {
        user_id: user.id.clone(),
        email: format!("forward+{}@mydomain.com", user.id),
    };
    
    Ok(Json(email))
}

pub async fn get_or_create_forwarding_email(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    user: AuthUser,
) -> Result<Json<ForwardingEmail>> {
    info!("Creating forwarding email for user {} (authenticated as {})", user_id, user.id);
    
    // Your handler logic here...
    let email = ForwardingEmail {
        user_id: user_id.clone(),
        email: format!("forward+{}@mydomain.com", user_id),
    };
    
    Ok(Json(email))
}
