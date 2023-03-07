use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct CliArgs {
    /// Binary name.
    pub bin: String,
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
}

impl CliArgs {
    /// Custom argument parser for escaping the '-' character.
    fn parse_arg(arg: &str) -> Result<String, String> {
        Ok(arg.replace("\\-", "-"))
    }
}
