use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;

const API_BASE_URL: &str = "https://api.bem.ai/v1-alpha";

#[derive(Parser)]
#[command(name = "bem")]
#[command(about = "CLI for interacting with bem.ai API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get resource by ID (auto-detects type from prefix: evt_, tr_)
    Get {
        /// Resource ID to retrieve
        id: String,
    },
    /// Manage events
    Events {
        #[command(subcommand)]
        action: EventsAction,
    },
    /// Manage transformations
    Transformations {
        #[command(subcommand)]
        action: TransformationsAction,
    },
}

#[derive(Subcommand)]
enum EventsAction {
    /// Get event details by ID
    Get {
        /// Event ID to retrieve
        id: String,
    },
}

#[derive(Subcommand)]
enum TransformationsAction {
    /// Get transformation details by ID
    Get {
        /// Transformation ID to retrieve
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { id } => {
            if id.starts_with("evt_") {
                get_event(&id).await?;
            } else if id.starts_with("tr_") {
                get_transformation(&id).await?;
            } else {
                anyhow::bail!("Unknown ID prefix. Expected 'evt_' for events or 'tr_' for transformations");
            }
        },
        Commands::Events { action } => match action {
            EventsAction::Get { id } => get_event(&id).await?,
        },
        Commands::Transformations { action } => match action {
            TransformationsAction::Get { id } => get_transformation(&id).await?,
        },
    }

    Ok(())
}

async fn get_event(event_id: &str) -> Result<()> {
    let api_token = std::env::var("BEM_API_TOKEN")
        .context("BEM_API_TOKEN environment variable must be set")?;

    let url = format!("{}/events/{}", API_BASE_URL, event_id);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("x-api-key", api_token)
        .send()
        .await
        .context("Failed to send request to bem.ai API")?;

    let status = response.status();
    
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("API request failed with status {}: {}", status, error_text);
    }

    let json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    let pretty_json = serde_json::to_string_pretty(&json)
        .context("Failed to format JSON")?;

    println!("{}", pretty_json);

    Ok(())
}

async fn get_transformation(transformation_id: &str) -> Result<()> {
    let api_token = std::env::var("BEM_API_TOKEN")
        .context("BEM_API_TOKEN environment variable must be set")?;

    let url = format!("https://api.bem.ai/v1-beta/transformations?transformationIDs={}", transformation_id);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("x-api-key", api_token)
        .send()
        .await
        .context("Failed to send request to bem.ai API")?;

    let status = response.status();
    
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
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
    let pretty_json = serde_json::to_string_pretty(transformation)
        .context("Failed to format JSON")?;

    println!("{}", pretty_json);

    Ok(())
}
