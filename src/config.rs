use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_api_token() -> Result<String> {
    if let Ok(token) = std::env::var("BEM_API_TOKEN") {
        return Ok(token);
    }

    let home_dir = std::env::var("HOME").context("Could not determine home directory")?;
    let bemrc_path = PathBuf::from(home_dir).join(".bemrc");

    if bemrc_path.exists() {
        dotenvy::from_path(&bemrc_path).ok();

        if let Ok(token) = std::env::var("BEM_API_TOKEN") {
            return Ok(token);
        }
    }

    anyhow::bail!(
        "BEM_API_TOKEN not found in environment and ~/.bemrc does not exist or does not contain BEM_API_TOKEN. \
        Please set BEM_API_TOKEN environment variable or create ~/.bemrc with BEM_API_TOKEN=<token>"
    )
}
