use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardingEmail {
    pub user_id: String,
    pub email: String,
}

impl ForwardingEmail {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            email: format!("forward+{}@mydomain.com", user_id),
        }
    }
}
