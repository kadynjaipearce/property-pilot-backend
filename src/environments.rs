use serde::Deserialize;
use shuttle_runtime::SecretStore;

#[derive(Debug, Clone, Deserialize)]
pub struct Environments {
    pub database_url: String,
    pub database_user: String,
    pub database_password: String,
    pub database_ns: String,
    pub database_db: String,
    pub auth0_domain: String,
    pub auth0_audience: String,
}

impl Environments {
    pub fn from_secrets(secrets: &SecretStore) -> Self {
        Self {
            database_url: secrets.get("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_user: secrets.get("DATABASE_USER").unwrap_or_else(|| "root".to_string()),
            database_password: secrets.get("DATABASE_PASSWORD").unwrap_or_else(|| "root".to_string()),
            database_ns: secrets.get("DATABASE_NS").unwrap_or_else(|| "test".to_string()),
            database_db: secrets.get("DATABASE_DB").unwrap_or_else(|| "test".to_string()),
            auth0_domain: secrets.get("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set"),
            auth0_audience: secrets.get("AUTH0_AUDIENCE").expect("AUTH0_AUDIENCE must be set"),
        }
    }
} 