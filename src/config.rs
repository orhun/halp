use crate::cli::{CliArgs, CliCommands};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Configuration.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Check the version flag.
    pub check_version: bool,
    /// Check the help flag.
    pub check_help: bool,
    /// Arguments to check.
    #[serde(rename = "check")]
    pub check_args: Option<Vec<Vec<String>>>,
    /// Command to run for manual pages.
    pub man_command: Option<String>,
    /// Pager to use for command outputs.
    pub pager_command: Option<String>,
}

impl Config {
    /// Checks the possible locations for the configuration file.
    ///
    /// - `<config_dir>/halp.toml`
    /// - `<config_dir>/halp/halp.toml`
    /// - `<config_dir>/halp/config`
    ///
    /// Returns the path if the configuration file is found.
    pub fn get_default_location() -> Option<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            let file_name = format!("{}.toml", env!("CARGO_PKG_NAME"));
            for config_file in vec![
                config_dir.join(&file_name),
                config_dir.join(env!("CARGO_PKG_NAME")).join(&file_name),
                config_dir.join(env!("CARGO_PKG_NAME")).join("config"),
            ] {
                if config_file.exists() {
                    return Some(config_file);
                }
            }
        }
        None
    }

    /// Parses the configuration file.
    pub fn parse(file: &Path) -> Result<Config> {
        let contents = fs::read_to_string(file)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Update the command-line arguments based on configuration.
    pub fn update_args(&self, cli_args: &mut CliArgs) {
        cli_args.no_help = !self.check_help;
        cli_args.no_version = !self.check_version;
        if let Some(man_command) = &self.man_command {
            if let Some(CliCommands::Plz {
                ref mut man_cmd, ..
            }) = cli_args.command
            {
                *man_cmd = man_command.clone();
            }
        }
        if let Some(pager_command) = &self.pager_command {
            if let Some(CliCommands::Plz { ref mut pager, .. }) = cli_args.command {
                *pager = Some(pager_command.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_parse_config() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join(format!("{}.toml", env!("CARGO_PKG_NAME")));
        if let Some(global_path) = Config::get_default_location() {
            path = global_path;
        }
        let mut config = Config::parse(&path)?;
        assert!(!config.check_help);
        config.check_help = true;
        config.man_command = Some(String::from("tldr"));
        let mut args = CliArgs {
            command: Some(CliCommands::default()),
            ..Default::default()
        };
        config.update_args(&mut args);
        assert!(!args.no_help);
        assert_eq!(
            "tldr",
            match args.command {
                Some(CliCommands::Plz { man_cmd, .. }) => {
                    man_cmd
                }
                _ => unreachable!(),
            }
        );
        Ok(())
    }
}
