use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

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
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = env::var("HOST") {
            if let Ok(parsed) = host.parse::<IpAddr>() {
                config.host = parsed;
            }
        }

        if let Ok(port) = env::var("PORT") {
            if let Ok(parsed) = port.parse::<u16>() {
                config.port = parsed;
            }
        }

        if let Ok(rust_log) = env::var("RUST_LOG") {
            if !rust_log.trim().is_empty() {
                config.rust_log = rust_log;
            }
        }

        config
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
