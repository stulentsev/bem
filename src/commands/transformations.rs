use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

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

pub async fn get_transformations_by_reference(
    references: Vec<String>,
    file: Option<PathBuf>,
    jsonl: bool,
) -> Result<()> {
    let mut collected: Vec<String> = references
        .into_iter()
        .map(|reference| reference.trim().to_string())
        .filter(|reference| !reference.is_empty())
        .collect();

    if let Some(path) = file {
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read references file {}", path.display()))?;

        collected.extend(contents.lines().filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }));
    }

    if collected.is_empty() {
        anyhow::bail!("No reference IDs provided. Supply positional references or use --file.");
    }

    let references_param = collected.join(",");

    let mut url = reqwest::Url::parse("https://api.bem.ai/v1-beta/transformations")
        .context("Failed to parse transformations endpoint URL")?;
    url.query_pairs_mut()
        .append_pair("referenceIDs", &references_param);

    let api_token = get_api_token()?;

    let client = reqwest::Client::new();
    let response = client
        .get(url)
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
        anyhow::bail!("No transformations found for the provided reference IDs");
    }

    if jsonl {
        for transformation in transformations {
            let line = serde_json::to_string(transformation)
                .context("Failed to format transformation as JSON")?;
            println!("{}", line);
        }
    } else {
        let output = serde_json::to_string(transformations)
            .context("Failed to format transformations as JSON")?;
        println!("{}", output);
    }

    Ok(())
}
