use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use explorer::preclude::*;
use tokio::signal;

async fn server(app: Router) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    debug!("Listening addr {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().merge(account::app());
    match server(app).await {
        Ok(()) => info!("Server Shutdown"),
        Err(err) => error!("Server Error, {}", err),
    }
}
