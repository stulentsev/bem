use anyhow::{Context, Result};
use serde_json::Value;

use crate::config::get_api_token;

pub async fn get_eval_results(transformation_id: &str) -> Result<()> {
    let api_token = get_api_token()?;

    let url = format!(
        "https://api.bem.ai/v1-beta/transformations/eval/results?transformationIDs={}",
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

    if let Some(results) = json["results"].as_object()
        && let Some(result) = results.get(transformation_id)
    {
        let pretty_json = serde_json::to_string_pretty(result).context("Failed to format JSON")?;
        println!("{}", pretty_json);
        return Ok(());
    }

    if let Some(pending) = json["pending"].as_array() {
        for item in pending {
            if item["transformationId"].as_str() == Some(transformation_id) {
                anyhow::bail!(
                    "Evaluation is still pending for transformation: {}",
                    transformation_id
                );
            }
        }
    }

    if let Some(failed) = json["failed"].as_array() {
        for item in failed {
            if item["transformationId"].as_str() == Some(transformation_id) {
                let error_msg = item["errorMessage"].as_str().unwrap_or("unknown error");
                anyhow::bail!(
                    "Evaluation failed for transformation {}: {}",
                    transformation_id,
                    error_msg
                );
            }
        }
    }

    anyhow::bail!(
        "No evaluation result found for transformation: {}",
        transformation_id
    );
}
