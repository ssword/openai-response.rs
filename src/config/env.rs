use anyhow::{anyhow, Result};
use std::env;

pub struct Config {
    pub api_key: String,
    pub default_model: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists (ignore errors if file doesn't exist)
        let _ = dotenvy::dotenv();

        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow!("OPENAI_API_KEY environment variable not set"))?;

        if api_key.trim().is_empty() {
            return Err(anyhow!("OPENAI_API_KEY environment variable is empty"));
        }

        Ok(Config {
            api_key,
            default_model: "gpt-4o-mini".to_string(),
        })
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn default_model(&self) -> &str {
        &self.default_model
    }
}
