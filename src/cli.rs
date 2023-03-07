use clap::Parser;

/// Command-line arguments.
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct CliArgs {
    /// Binary name.
    pub bin: String,
}
