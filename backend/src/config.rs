use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8000)?
            .set_default("database.url", "sqlite://./data.db")?
            .set_default("jwt.expires_in", 86400)?
            .add_source(config::Environment::with_prefix("ROWAN"))
            .build()?;

        let mut config: AppConfig = settings.try_deserialize()?;
        
        // JWT secret 必须从环境变量获取
        config.jwt.secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        Ok(config)
    }
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
