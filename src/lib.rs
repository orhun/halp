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
use cli::Commands;
use colored::*;
use config::Config;
use std::io::Write;
use std::process::{Command, Stdio};

/// Checks if the given arguments exist.
fn check_args<'a, ArgsIter: Iterator<Item = &'a str>, Output: Write>(
    bin: &str,
    args: ArgsIter,
    verbose: bool,
    output: &mut Output,
) -> Result<()> {
    for arg in args {
        let command = format!("{} {}", bin, arg);
        writeln!(
            output,
            "{}  {} '{}'",
            "(°ロ°)".magenta(),
            "checking".green().bold(),
            command.white().italic()
        )?;
        let cmd_out = Command::new("script")
            .args(&[
                String::from("-q"),
                String::from("-e"),
                String::from("-c"),
                command,
                String::from("/dev/null"),
            ])
            .stderr(Stdio::inherit())
            .output()?;
        if cmd_out.status.success() {
            writeln!(
                output,
                "{} {} '{}' argument found!",
                "\\(^ヮ^)/".magenta(),
                "success".cyan().bold(),
                arg.white().italic()
            )?;
            output.write_all(&cmd_out.stdout)?;
            break;
        } else {
            writeln!(
                output,
                "{}     {} '{}' argument not found.",
                "(×﹏×)".magenta(),
                "error".red().bold(),
                arg.white().italic()
            )?;
            if verbose {
                writeln!(
                    output,
                    "{}      {}",
                    "(o_O)".magenta(),
                    "debug".yellow().bold(),
                )?;
                if !cmd_out.stdout.is_empty() {
                    writeln!(output, "{}:", "stdout".white().italic())?;
                    output.write_all(&cmd_out.stdout)?;
                }
                if !cmd_out.stderr.is_empty() {
                    writeln!(output, "{}:", "stderr".white().italic())?;
                    output.write_all(&cmd_out.stderr)?;
                }
            }
        }
    }
    Ok(())
}

/// Shows command-line help about the given binary.
pub fn get_help<Output: Write>(
    config: Option<Config>,
    bin: &str,
    cli_args: &CliArgs,
    output: &mut Output,
) -> Result<()> {
    if let Some(config_args) = config.and_then(|v| v.check_args) {
        for args in config_args {
            check_args(
                bin,
                args.iter().map(|v| v.as_str()),
                cli_args.verbose,
                output,
            )?;
        }
        return Ok(());
    }
    if let Some(ref args) = cli_args.check_args {
        check_args(
            bin,
            args.iter().map(|v| v.as_str()),
            cli_args.verbose,
            output,
        )?;
        return Ok(());
    }
    for arg_variants in [
        (!cli_args.no_version).then(VersionArg::variants),
        (!cli_args.no_help).then(HelpArg::variants),
    ]
    .iter()
    .flatten()
    {
        check_args(
            bin,
            arg_variants.iter().map(|v| v.as_str()),
            cli_args.verbose,
            output,
        )?;
    }
    Ok(())
}

/// Runs `halp`.
pub fn run<Output: Write>(mut cli_args: CliArgs, output: &mut Output) -> Result<()> {
    let config = if let Some(config_file) = cli_args
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
    if let Some(ref bin) = cli_args.bin {
        get_help(config, bin, &cli_args, output)?;
    } else if let Some(Commands::Plz { bin: _ }) = cli_args.command {
        todo!();
    }
    Ok(())
}
