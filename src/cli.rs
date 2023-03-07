use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct CliArgs {
    /// Binary name.
    pub bin: String,
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
