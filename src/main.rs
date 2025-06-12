mod database;
mod error;

use axum::{
    extract::State,
    routing::get,
    Router,
};
use error::Result;

// Example route that uses the database
async fn hello_world(
    State(db): State<database::database::Database>
) -> Result<&'static str> {

    Ok("Hello, world!")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let db = database::database::Database::new("ws://localhost:8000").await?;

    let router = Router::new()
        .route("/", get(hello_world))
        .with_state(db);

    Ok(router.into())
}
