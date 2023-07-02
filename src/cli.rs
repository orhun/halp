use crate::config::Config;
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
        #[arg(short, long)]
        man_cmd: Option<String>,
        /// Use a custom URL for cheat.sh.
        #[arg(long, env = "CHEAT_SH_URL", value_name = "URL")]
        cheat_sh_url: Option<String>,
        /// Sets the pager to use.
        #[arg(short, long)]
        pager: Option<String>,
        /// Disables the pager.
        #[arg(long)]
        no_pager: bool,
    },
}

impl CliArgs {
    /// Custom argument parser for escaping the '-' character.
    fn parse_arg(arg: &str) -> Result<String, String> {
        Ok(arg.replace("\\-", "-"))
    }

    /// Update the configuration based on the command-line arguments (the command-line arguments will override the configuration).
    pub fn update_conf(&self, config: &mut Config) {
        config.check_help = !self.no_help;
        config.check_version = !self.no_version;
        if let Some(CliCommands::Plz {
            ref man_cmd,
            ref cheat_sh_url,
            no_pager,
            ref pager,
            ..
        }) = self.subcommand
        {
            if let Some(man_cmd) = man_cmd {
                config.man_command = man_cmd.clone();
            }
            if let Some(cheat_sh_url) = cheat_sh_url {
                config.cheat_sh_url = cheat_sh_url.clone();
            }
            if no_pager {
                config.pager_command = None;
            } else if let Some(pager) = pager {
                config.pager_command = Some(pager.clone());
            }
        }
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

    #[test]
    fn test_update_conf() {
        let mut config = Config::default();
        let args = CliArgs {
            subcommand: Some(CliCommands::Plz {
                cmd: "ps".to_string(),
                pager: Some("bat".to_string()),
                cheat_sh_url: None,
                man_cmd: None,
                no_pager: false,
            }),
            ..Default::default()
        };
        args.update_conf(&mut config);
        assert!(config.check_help);
        assert_eq!(Some(String::from("bat")), config.pager_command);
    }
}
