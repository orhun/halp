use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Debug, Default, Parser)]
#[command(
    version,
    author,
    about,
    subcommand_negates_reqs = true,
    disable_help_subcommand = true
)]
pub struct CliArgs {
    /// Binary name.
    #[arg(required = true)]
    pub bin: Option<String>,
    /// Sets the argument to check.
    #[arg(long = "check", value_name = "ARG", value_parser = CliArgs::parse_arg)]
    pub check_args: Option<Vec<String>>,
    /// Disable checking the version information.
    #[arg(long)]
    pub no_version: bool,
    /// Disable checking the help information.
    #[arg(long)]
    pub no_help: bool,
    /// Sets the configuration file.
    #[arg(short, long, env = "HALP_CONFIG", value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Enables verbose logging.
    #[arg(short, long)]
    pub verbose: bool,
    /// Subcommands.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get additional help.
    Plz {
        /// Binary name.
        bin: String,
    },
}

impl CliArgs {
    /// Custom argument parser for escaping the '-' character.
    fn parse_arg(arg: &str) -> Result<String, String> {
        Ok(arg.replace("\\-", "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_args() {
        CliArgs::command().debug_assert()
    }
}
