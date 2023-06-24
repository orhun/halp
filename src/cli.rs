use clap::builder::ArgPredicate;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line arguments.
#[derive(Debug, Default, Parser)]
#[command(
version,
author,
about,
subcommand_negates_reqs = true,
disable_help_subcommand = true,
override_usage = format!("
  {bin} [OPTIONS] <CMD>
  {bin} [OPTIONS] <COMMAND> <CMD>", bin = env!("CARGO_PKG_NAME"))
)]
pub struct CliArgs {
    /// Command or binary name.
    #[arg(required = true)]
    pub cmd: Option<String>,
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
    pub subcommand: Option<CliCommands>,
}

/// Subcommands.
#[derive(Debug, Subcommand)]
pub enum CliCommands {
    /// Get additional help.
    Plz {
        /// Command or binary name.
        cmd: String,
        /// Sets the manual page command to run.
        #[arg(short, long, default_value = "man")]
        man_cmd: String,
        /// Cheat.sh URL.
        #[arg(long, env = "CHEAT_SH_URL", value_name = "URL")]
        cheat_sh_url: Option<String>,
        /// Sets the pager to use.
        #[arg(
        short,
        long,
        default_value = "less -R",
        default_value_if("no_pager", ArgPredicate::IsPresent, None)
        )]
        pager: Option<String>,
        /// Disables the pager.
        #[arg(long)]
        no_pager: bool,
    },
}

impl Default for CliCommands {
    fn default() -> Self {
        CliCommands::Plz {
            cmd: String::new(),
            man_cmd: String::from("man"),
            cheat_sh_url: None,
            pager: Some(String::from("less")),
            no_pager: false,
        }
    }
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_cli_args() {
        CliArgs::command().debug_assert();
        assert_eq!(Ok("--help"), CliArgs::parse_arg("\\--help").as_deref());
    }
}
