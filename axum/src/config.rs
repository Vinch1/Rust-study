use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub rust_log: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
            rust_log: String::from("info,axum=info,tower_http=info"),
        }
    }
}

impl Config {
    pub fn resolve_rust_log() -> String {
        match env::var("RUST_LOG") {
            Ok(rust_log) if !rust_log.trim().is_empty() => rust_log,
            _ => Self::default().rust_log,
        }
    }

    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.rust_log = Self::resolve_rust_log();

        if let Ok(host) = env::var("HOST") {
            match host.parse::<IpAddr>() {
                Ok(parsed) => {
                    config.host = parsed;
                }
                Err(err) => {
                    warn!(host = %host, error = %err, "invalid HOST value; using default");
                }
            }
        }

        if let Ok(port) = env::var("PORT") {
            match port.parse::<u16>() {
                Ok(parsed) => {
                    config.port = parsed;
                }
                Err(err) => {
                    warn!(port = %port, error = %err, "invalid PORT value; using default");
                }
            }
        }

        info!(
            host = %config.host,
            port = config.port,
            rust_log = %config.rust_log,
            "configuration loaded"
        );

        config
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
