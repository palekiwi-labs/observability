use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, env = "PORT", default_value = "3000")]
    pub port: u16,

    #[arg(long, env = "DATABASE_URL", default_value = "sqlite:observability.db")]
    pub database_url: String,

    #[arg(long, env = "NOTIFICATION_URL")]
    pub notification_url: Option<String>,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self::parse())
    }
}
