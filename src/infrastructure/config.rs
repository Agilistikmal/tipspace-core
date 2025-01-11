use std::sync::Arc;

use clap::Parser;

#[derive(Parser)]
pub struct Config {
    /// PostgreSQL database url connection
    #[clap(env)]
    pub database_url: String,
}

impl Config {
    pub fn load() -> Arc<Self> {
        dotenvy::dotenv().ok();
        Arc::new(Config::parse())
    }
}
