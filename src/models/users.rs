use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
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
