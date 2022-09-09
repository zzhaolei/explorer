use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use launcher::prelude::*;
use tokio::signal;

async fn server(app: Router) -> Result<()> {
    let addr = SocketAddr::from((CONFIG.http.host, CONFIG.http.port));

    info!("Listening addr {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("无法监听Ctrl+C信号");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("无法监听信号")
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
    info!("enabled `{}` feature.", &CONFIG.env_flag);

    let app = Router::new().merge(auth::app());

    match server(app).await {
        Ok(()) => info!("Server Shutdown"),
        Err(err) => error!("Server Error, {}", err),
    }
}
