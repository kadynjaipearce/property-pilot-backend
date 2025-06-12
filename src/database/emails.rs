use crate::models::users::ForwardingEmail;
use crate::error::{Result, AppError};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn get_or_create_forwarding_email(
    db: &Surreal<Client>,
    user_id: &str,
) -> Result<ForwardingEmail> {
    // Try to get existing email
    let existing: Option<ForwardingEmail> = db
        .select(("user_emails", user_id))
        .await?;

    if let Some(email) = existing {
        return Ok(email);
    }

    // Create new email if none exists
    let created: Option<ForwardingEmail> = db
        .create(("user_emails", user_id))
        .content(ForwardingEmail::new(user_id))
        .await?;

    created.ok_or_else(|| AppError::Internal("Failed to create forwarding email".to_string()))
} 