use crate::error::AppError;

use surrealdb::{engine::remote::ws::Client, engine::remote::ws::Wss, opt::auth::Root, Surreal};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    #[allow(dead_code)]
    pub async fn new(url: &str) -> Result<Self, AppError> {
        let client = Surreal::new::<Wss>(url).await?;

        client
            .signin(Root {
                username: "test",
                password: "test",
            })
            .await?;

        client.use_ns("test").use_db("test").await?;

        client.query("").await?;

        Ok(Self { client })
    }

    #[allow(dead_code)]
    pub fn client(&self) -> &Surreal<Client> {
        &self.client
    }
}
