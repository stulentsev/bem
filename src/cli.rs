use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bem")]
#[command(about = "CLI for interacting with bem.ai API", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get resource by ID (auto-detects type from prefix: evt_, tr_)
    Get {
        /// Resource ID to retrieve
        id: String,
    },
    /// Get evaluation results for a transformation
    Eval {
        /// Transformation ID to get evaluation results for
        transformation_id: String,
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
pub enum EventsAction {
    /// Get event details by ID
    Get {
        /// Event ID to retrieve
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TransformationsAction {
    /// Get transformation details by ID
    Get {
        /// Transformation ID to retrieve
        id: String,
    },
}
