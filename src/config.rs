use std::sync::Arc;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: Arc<str>,
    pub port: u16,
    pub debug: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, dotenv::Error> {
        dotenv::dotenv().ok();
        let database_url = dotenv::var("DATABASE_URL")?.into();
        let port = dotenv::var("PORT")
            .unwrap_or_else(|_| "8000".to_owned())
            .parse()
            .unwrap_or(8000);
        let debug = dotenv::var("DEBUG")
            .unwrap_or_else(|_| "false".to_owned())
            .parse()
            .unwrap_or(false);
        Ok(Config {
            database_url,
            port,
            debug,
        })
    }
}

