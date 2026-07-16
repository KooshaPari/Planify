use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "agileplus")]
#[command(about = "AgilePlus CLI")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a natural language prompt into an intent graph
    Intent(commands::intent::IntentCommand),
    /// Validate an intent graph JSON file
    Validate(commands::validate::ValidateCommand),
    /// Query nodes from an intent graph
    Query(commands::query::QueryCommand),
    /// Store an intent graph into a SQLite database
    Store(commands::store::StoreCommand),
    /// List all graph ids stored in a SQLite database
    List(commands::list::ListCommand),
    /// Load a stored graph and write it as JSON
    Dump(commands::dump::DumpCommand),
    /// Delete a graph from a SQLite database
    Delete(commands::delete::DeleteCommand),
    /// Export all intent graphs from a SQLite database to individual JSON files
    ExportAll(commands::export::ExportAll),
    /// Import intent graphs from JSON files into a SQLite database
    ImportAll(commands::import::ImportAll),
    /// Manage tags on a stored graph
    Tag(commands::tag::TagCommand),
    /// Manage free-form notes on a stored graph
    Note(commands::note::NoteCommand),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Intent(cmd) => cmd.run(),
        Commands::Validate(cmd) => cmd.run(),
        Commands::Query(cmd) => cmd.run(),
        Commands::Store(cmd) => cmd.run(),
        Commands::List(cmd) => cmd.run(),
        Commands::Dump(cmd) => cmd.run(),
        Commands::Delete(cmd) => cmd.run(),
        Commands::ExportAll(cmd) => cmd.run(),
        Commands::ImportAll(cmd) => cmd.run(),
        Commands::Tag(cmd) => cmd.run(),
        Commands::Note(cmd) => cmd.run(),
    }
}
