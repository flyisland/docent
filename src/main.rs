mod cli;
mod commands;
mod model;
mod output;
mod rules;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    let code = match cli.command {
        cli::Command::Init => commands::init::run(),
        cli::Command::Lint(args) => commands::lint::run(args),
        cli::Command::Status => commands::status::run(),
        cli::Command::Docs(args) => commands::docs::run(args),
    };
    std::process::exit(code);
}
