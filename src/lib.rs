//! A CLI tool to get help with CLI tools.

#![warn(missing_docs, clippy::unwrap_used)]

/// Common command-line arguments.
pub mod args;

/// Command-line argument parser.
pub mod cli;

/// Error handling implementation.
pub mod error;

/// Configuration.
pub mod config;

use crate::args::{HelpArg, VersionArg};
use crate::cli::CliArgs;
use crate::error::Result;
use config::Config;
use std::io::Write;
use std::process::Command;

/// Check if the argument exists.
fn check_argument<'a, ArgsIter: Iterator<Item = &'a str>, Output: Write>(
    bin: &str,
    args: ArgsIter,
    output: &mut Output,
) -> Result<()> {
    for arg in args {
        let cmd_out = Command::new("script")
            .args(&[
                String::from("-q"),
                String::from("-e"),
                String::from("-c"),
                format!("{} {}", bin, arg),
                String::from("/dev/null"),
            ])
            .output()?;
        if cmd_out.status.success() {
            writeln!(output, "Argument found.")?;
            output.write_all(&cmd_out.stdout)?;
            break;
        } else {
            writeln!(output, "Argument not found.")?;
        }
    }
    Ok(())
}

/// Runs `halp`.
pub fn run<Output: Write>(mut cli_args: CliArgs, output: &mut Output) -> Result<()> {
    if let Some(config_file) = cli_args
        .config
        .to_owned()
        .or_else(Config::get_default_location)
    {
        let config = Config::parse(&config_file)?;
        config.update_args(&mut cli_args);
        Some(config)
    } else {
        None
    };
    for arg_variants in [
        (!cli_args.no_version).then(VersionArg::variants),
        (!cli_args.no_help).then(HelpArg::variants),
    ]
    .iter()
    .flatten()
    {
        check_argument(
            &cli_args.bin,
            arg_variants.iter().map(|v| v.as_str()),
            output,
        )?;
    }
    Ok(())
}
