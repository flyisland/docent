use clap::builder::{PossibleValue, TypedValueParser};
use clap::{Args, Parser, Subcommand};

pub const STATUS_VALUES: [&str; 5] = ["Draft", "Accepted", "Rejected", "Superseded", "Deprecated"];

#[derive(Clone, Copy, Debug)]
pub struct CaseInsensitiveStatus;

impl TypedValueParser for CaseInsensitiveStatus {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = match value.to_str() {
            Some(v) => v,
            None => {
                return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue).with_cmd(cmd));
            }
        };
        let lower = value.to_ascii_lowercase();
        if let Some(canon) = STATUS_VALUES
            .iter()
            .find(|v| v.to_ascii_lowercase() == lower)
        {
            return Ok((*canon).to_string());
        }
        let mut err = clap::Error::new(clap::error::ErrorKind::InvalidValue).with_cmd(cmd);
        if let Some(arg) = arg {
            err.insert(
                clap::error::ContextKind::InvalidArg,
                clap::error::ContextValue::String(arg.to_string()),
            );
        }
        err.insert(
            clap::error::ContextKind::InvalidValue,
            clap::error::ContextValue::String(value.to_string()),
        );
        err.insert(
            clap::error::ContextKind::ValidValue,
            clap::error::ContextValue::Strings(
                STATUS_VALUES.iter().map(|s| s.to_string()).collect(),
            ),
        );
        Err(err)
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            STATUS_VALUES.iter().copied().map(PossibleValue::new),
        ))
    }
}

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
    /// List RFCs or ADRs, optionally filtered by status
    List(ListArgs),
    /// Read or explicitly export Docent's bundled canonical documentation specification
    Docs(DocsArgs),
}

#[derive(Args)]
pub struct DocsArgs {
    #[command(subcommand)]
    pub command: DocsCommand,
}

#[derive(Subcommand)]
pub enum DocsCommand {
    /// Write the complete bundled canonical specification to standard output
    Show,
    /// Export the complete bundled canonical specification without overwriting a file
    Export {
        /// Destination file (or a directory, which receives software-project-documentation-specification.md)
        destination: std::path::PathBuf,
        /// Treat an extensionless destination as a file rather than a directory
        #[arg(long)]
        file: bool,
    },
    /// Read or explicitly export the latest project operational-policy template
    ProjectPolicy(ProjectPolicyArgs),
}

#[derive(Args)]
pub struct ProjectPolicyArgs {
    #[command(subcommand)]
    pub command: ProjectPolicyCommand,
}

#[derive(Subcommand)]
pub enum ProjectPolicyCommand {
    /// Write the latest project operational-policy template to standard output
    Show,
    /// Export the latest project operational-policy template without overwriting a file
    Export {
        /// Destination file (or a directory, which receives project-docs-readme.md)
        destination: std::path::PathBuf,
        /// Treat an extensionless destination as a file rather than a directory
        #[arg(long)]
        file: bool,
    },
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

#[derive(Args)]
pub struct ListArgs {
    /// Which document type to list
    #[arg(value_parser = ["rfc", "adr"])]
    pub doc_type: String,

    /// Only list documents in this status (case-insensitive)
    #[arg(short, long, value_parser = clap::builder::ValueParser::new(CaseInsensitiveStatus))]
    pub status: Option<String>,

    /// Only list ADRs in this implementation state
    #[arg(short, long, value_parser = ["implemented", "pending"])]
    pub implementation: Option<String>,
}
