mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands, EventsAction, TransformationsAction};
use crate::commands::{get_eval_results, get_event, get_transformation};

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
                anyhow::bail!(
                    "Unknown ID prefix. Expected 'evt_' for events or 'tr_' for transformations"
                );
            }
        }
        Commands::Eval { transformation_id } => get_eval_results(&transformation_id).await?,
        Commands::Events { action } => match action {
            EventsAction::Get { id } => get_event(&id).await?,
        },
        Commands::Transformations { action } => match action {
            TransformationsAction::Get { id } => get_transformation(&id).await?,
        },
    }

    Ok(())
}
