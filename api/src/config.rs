use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Clone)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "DATABASE_MAX_CONNECTIONS", default_value = "50")]
    pub database_max_connections: u32,
    
    /// The HMAC signing and verification key used for login tokens (JWTs).
    #[arg(long, env = "HMAC_KEY")]
    pub hmac_key: String,

    /// The port this server will listen on.
    #[arg(long, env = "PORT", default_value = "8080")]
    pub port: u16,

    /// The host address to bind to.
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,

    /// Log level configuration.
    #[arg(long, env = "RUST_LOG", default_value = "spaced_repetition_api=debug,tower_http=debug")]
    pub log_level: String,
}

impl Config {
    /// Get the socket address for the server
    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Failed to parse host and port into socket address")
    }

    pub fn setup_logging(&self) {
        // If RUST_LOG is set in the environment, it will take precedence
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", &self.log_level);
        }
        
        env_logger::init();
    }
}