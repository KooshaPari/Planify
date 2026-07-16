use agileplus_domain::intent_graph::IntentGraph;
use agileplus_trace_validator::validate_intent_graph;
use anyhow::{Context, Result};
use clap::Args;

/// Validate an intent graph JSON file.
#[derive(Debug, Args)]
#[command(name = "validate")]
pub struct ValidateCommand {
    /// Path to the JSON graph file.
    #[arg(short, long)]
    input: String,
}

impl ValidateCommand {
    pub fn run(&self) -> Result<()> {
        let json = std::fs::read_to_string(&self.input)
            .with_context(|| format!("Failed to read input: {}", self.input))?;

        let graph: IntentGraph = serde_json::from_str(&json)
            .with_context(|| format!("Failed to parse JSON from: {}", self.input))?;

        match validate_intent_graph(&graph) {
            Ok(()) => {
                println!("Validation passed.");
                println!("  Nodes: {}", graph.nodes.len());
                println!("  Edges: {}", graph.edges.len());
            }
            Err(errors) => {
                eprintln!("Validation failed with {} error(s):", errors.len());
                for err in errors {
                    eprintln!("  - {}", err);
                }
                std::process::exit(1);
            }
        }

        Ok(())
    }
}
