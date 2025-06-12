use crate::error::AppError;
use crate::environments::Environments;

use surrealdb::{engine::{remote::ws::{Client, Wss, Ws}}, opt::auth::Root, Surreal};

#[derive(Clone, Debug)]
pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn new(env: &Environments) -> Result<Self, AppError> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        client
            .signin(Root {
                username: &env.database_user,
                password: &env.database_password,
            })
            .await?;

        client
            .use_ns(&env.database_ns)
            .use_db(&env.database_db)
            .await?;

        Ok(Self { client })
    }

    pub fn client(&self) -> &Surreal<Client> {
        &self.client
    }
}
