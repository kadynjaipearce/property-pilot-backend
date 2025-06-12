use axum::{
    extract::{Path, State},
    Json,
};
use crate::models::users::ForwardingEmail;
use crate::error::Result;
use crate::AppState;

pub async fn get_user(State(state): State<AppState>) -> Result<Json<ForwardingEmail>> {
    // Placeholder for user retrieval logic
    let user = ForwardingEmail {
        user_id: "1".to_string(),
        email: "forward+1@mydomain.com".to_string(),
    };
    Ok(Json(user))
}

pub async fn get_or_create_forwarding_email(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<ForwardingEmail>> {
    let email = crate::database::emails::get_or_create_forwarding_email(state.db.client(), &user_id).await?;
    Ok(Json(email))
}
