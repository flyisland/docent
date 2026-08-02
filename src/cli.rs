use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "docent",
    version,
    about = "Mechanical documentation consistency checks for RFC / ADR / architecture / CONTEXT / AGENTS"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create the standard directory structure and template files in the current directory
    Init,
    /// Run the lint rules against the project in the current directory
    Lint(LintArgs),
    /// Summarize the current state of the project's documentation
    Status,
}

#[derive(Args)]
pub struct LintArgs {
    /// Print structured JSON output instead of human-readable text
    #[arg(long)]
    pub json: bool,

    /// Apply the mechanical fixes defined in the implementation spec
    #[arg(long)]
    pub fix: bool,
}
