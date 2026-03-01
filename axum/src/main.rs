mod config;
mod error;
mod handlers;
mod router;
mod state;

use std::error::Error;

use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::{config::Config, router::build_router, state::AppState};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        error!(error = %err, "server exited with error");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let initial_log_filter = Config::resolve_rust_log();
    init_tracing(&initial_log_filter);

    let config = Config::from_env();

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await?;

    let state = AppState::new("axum-service", env!("CARGO_PKG_VERSION"));
    info!(
        service = state.service_name,
        version = state.service_version,
        "application initialized"
    );

    let app = build_router(state);

    info!(%addr, "server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_tracing(default_filter: &str) {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_filter))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            error!(error = %err, "failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(err) => {
                error!(error = %err, "failed to install terminate signal handler");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received");
}
