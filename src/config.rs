use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub cookies: String,
    pub convex_session_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "11434".to_string())
            .parse()
            .context("Invalid PORT")?;

        let cookies = std::env::var("T3_COOKIES")
            .context("T3_COOKIES environment variable required")?;

        let convex_session_id = std::env::var("T3_CONVEX_SESSION_ID")
            .context("T3_CONVEX_SESSION_ID environment variable required")?;

        Ok(Self {
            port,
            cookies,
            convex_session_id,
        })
    }
}
