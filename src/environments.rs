use serde::Deserialize;
use shuttle_runtime::SecretStore;

#[derive(Debug, Clone, Deserialize)]
pub struct Environments {
    pub database_url: String,
    pub database_user: String,
    pub database_password: String,
    pub database_ns: String,
    pub database_db: String,
}

impl Environments {
    pub fn from_secrets(secrets: &SecretStore) -> Self {
        Self {
            database_url: secrets.get("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_user: secrets.get("DATABASE_USER").expect("DATABASE_USER must be set"),
            database_password: secrets.get("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            database_ns: secrets.get("DATABASE_NS").expect("DATABASE_NS must be set"),
            database_db: secrets.get("DATABASE_DB").expect("DATABASE_DB must be set"),
        }
    }
} 