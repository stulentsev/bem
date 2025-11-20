use anyhow::{Context, Result};
use serde_json::Value;

use crate::config::get_api_token;

pub async fn get_transformation(transformation_id: &str) -> Result<()> {
    let api_token = get_api_token()?;

    let url = format!(
        "https://api.bem.ai/v1-beta/transformations?transformationIDs={}",
        transformation_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("x-api-key", api_token)
        .send()
        .await
        .context("Failed to send request to bem.ai API")?;

    let status = response.status();

    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("API request failed with status {}: {}", status, error_text);
    }

    let json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    let transformations = json["transformations"]
        .as_array()
        .context("Response does not contain 'transformations' array")?;

    if transformations.is_empty() {
        anyhow::bail!("No transformation found with ID: {}", transformation_id);
    }

    let transformation = &transformations[0];
    let pretty_json =
        serde_json::to_string_pretty(transformation).context("Failed to format JSON")?;

    println!("{}", pretty_json);

    Ok(())
}
