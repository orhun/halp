//! A CLI tool to get help with CLI tools.

#![warn(missing_docs, clippy::unwrap_used)]

/// Common command-line arguments.
pub mod args;

/// Command-line argument parser.
pub mod cli;

/// Error handling implementation.
pub mod error;

use crate::args::{HelpArg, VersionArg};
use crate::cli::CliArgs;
use crate::error::Result;
use std::io::Write;
use std::process::Command;

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
pub fn run<Output: Write>(cli_args: CliArgs, output: &mut Output) -> Result<()> {
    for arg_variants in [
        (!cli_args.no_version).then(|| VersionArg::variants()),
        (!cli_args.no_help).then(|| HelpArg::variants()),
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
