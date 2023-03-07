//! A CLI tool to get help with CLI tools.

#![warn(missing_docs, clippy::unwrap_used)]

/// Common command-line arguments.
pub mod args;

/// Command-line argument parser.
pub mod cli;

/// Error handling implementation.
pub mod error;

use args::HelpArg;

use crate::cli::CliArgs;
use crate::error::Result;
use std::io::Write;
use std::process::Command;

/// Runs `halp`.
pub fn run<Output: Write>(cli_args: CliArgs, output: &mut Output) -> Result<()> {
    for arg in HelpArg::variants() {
        let cmd_out = Command::new("script")
            .args(&[
                String::from("-q"),
                String::from("-e"),
                String::from("-c"),
                format!("{} {}", cli_args.bin, arg),
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
