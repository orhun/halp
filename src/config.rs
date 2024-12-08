use crate::error::Result;
use crate::helper::args::common::{HelpArg, VersionArg};
use crate::helper::args::FOUND_EMOTICON;
use crate::helper::docs::cheat_sh::DEFAULT_CHEAT_SHEET_PROVIDER;
use crate::helper::docs::cheatsheets::DEFAULT_CHEATSHEETS_PROVIDER;
use crate::helper::docs::eg::DEFAULT_EG_PAGES_PROVIDER;
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Check the version flag.
    pub check_version: bool,
    /// Check the help flag.
    pub check_help: bool,
    /// Arguments to check.
    #[serde(rename = "check")]
    pub check_args: Option<Vec<Vec<String>>>,
    /// Command to run for manual pages.
    pub man_command: String,
    /// Pager to use for command outputs, None to disable.
    pub pager_command: Option<String>,
    /// Use a custom URL for cheat.sh.
    pub cheat_sh_url: Option<String>,
    /// Use a custom URL for `eg` pages provider.
    pub eg_url: Option<String>,
    /// Use a custom URL for cheatsheets provider.
    pub cheatsheets_url: Option<String>,
    /// Timeout for running the commands.
    pub timeout: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            check_version: true,
            check_help: true,
            check_args: Some(vec![
                VersionArg::variants()
                    .iter()
                    .map(|s| s.as_str().to_string())
                    .collect(),
                HelpArg::variants()
                    .iter()
                    .map(|s| s.as_str().to_string())
                    .collect(),
            ]),
            man_command: "man".to_string(),
            pager_command: Some("less -R".to_string()),
            cheat_sh_url: Some(DEFAULT_CHEAT_SHEET_PROVIDER.to_string()),
            eg_url: Some(DEFAULT_EG_PAGES_PROVIDER.to_string()),
            cheatsheets_url: Some(DEFAULT_CHEATSHEETS_PROVIDER.to_string()),
            timeout: Some(5),
        }
    }
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
        if let Some(config_dirs) = Config::get_default_locations() {
            for config_dir in config_dirs {
                if config_dir.exists() {
                    return Some(config_dir);
                }
            }
        }
        None
    }

    #[inline(always)]
    fn get_default_locations() -> Option<Vec<PathBuf>> {
        if let Some(config_dir) = dirs::config_dir() {
            let file_name = concat!(env!("CARGO_PKG_NAME"), ".toml");
            return Some(vec![
                config_dir.join(&file_name),
                config_dir.join(env!("CARGO_PKG_NAME")).join(&file_name), // XDG style
                config_dir.join(env!("CARGO_PKG_NAME")).join("config"),
            ]);
        }
        None
    }

    /// Parses the configuration file.
    pub fn parse(file: &Path) -> Result<Config> {
        let contents = fs::read_to_string(file)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Writes the configuration file to the default location (XDG style).
    pub fn write<Output: Write>(&self, output: &mut Output) -> Result<()> {
        if let Some(config_dirs) = Config::get_default_locations() {
            let xdg_conf_path = &config_dirs[1];
            if let Some(parent) = xdg_conf_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let contents = toml::to_string(&self)?;
            writeln!(
                output,
                "{} {} {}",
                FOUND_EMOTICON.magenta(),
                "writing the default configuration to".green().bold(),
                format!("{:?}", xdg_conf_path).white().italic()
            )?;
            fs::write(xdg_conf_path, contents)?;
        }
        Ok(())
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
            .join(concat!(env!("CARGO_PKG_NAME"), ".toml"));
        if let Some(global_path) = Config::get_default_location() {
            path = global_path;
        }
        let config = Config::parse(&path)?;
        assert!(config.check_help);
        assert!(config.check_version);
        assert_eq!(
            config.cheat_sh_url,
            Some(DEFAULT_CHEAT_SHEET_PROVIDER.to_string())
        );
        Ok(())
    }
}
