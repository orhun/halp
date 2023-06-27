//! A CLI tool to get help with CLI tools 🐙

#![warn(missing_docs, clippy::unwrap_used)]

/// Help implementation.
pub mod helper;

/// Command-line argument parser.
pub mod cli;

/// Error handling implementation.
pub mod error;

/// Configuration.
pub mod config;

use crate::cli::CliArgs;
use crate::error::Result;
use cli::CliCommands;
use config::Config;
use helper::args::get_args_help;
use helper::docs::get_docs_help;
use std::io::Write;

/// Runs `halp`.
pub fn run<Output: Write>(mut cli_args: CliArgs, output: &mut Output) -> Result<()> {
    let mut config = if let Some(config_file) = cli_args
        .config
        .to_owned()
        .or_else(Config::get_default_location)
    {
        Config::parse(&config_file)?
    } else {
        Config::default()
        //  TODO: Create a default config file.
    };

    cli_args.update_conf(&mut config);

    if let Some(ref cmd) = cli_args.cmd {
        get_args_help(cmd, &config, cli_args.verbose, output)?;
    } else if let Some(CliCommands::Plz { ref cmd, .. }) = cli_args.subcommand {
        get_docs_help(cmd, &config, output)?;
    }
    Ok(())
}
