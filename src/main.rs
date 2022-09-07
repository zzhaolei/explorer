use std::net::SocketAddr;

use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router};

async fn login() -> impl IntoResponse {
    "hello world"
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = Router::new().route("/login", get(login));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Listening addr {:?}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
