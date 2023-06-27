use crate::error::Result;
use crate::helper::docs::cheat::DEFAULT_CHEAT_SHEET_PROVIDER;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
    pub cheat_sh_url: String,
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
            let file_name = format!("{}.toml", env!("CARGO_PKG_NAME"));
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
    pub fn write(&self) -> Result<()> {
        if let Some(config_dirs) = Config::get_default_locations() {
            let xdg_conf_path = &config_dirs[1];
            if let Some(parent) = xdg_conf_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let contents = toml::to_string(&self)?;
            fs::write(xdg_conf_path, contents)?;
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            check_version: true,
            check_help: true,
            check_args: Some(vec![
                vec!["-v".to_string(), "-V".to_string(), "--version".to_string()],
                vec![
                    "-h".to_string(),
                    "--help".to_string(),
                    "help".to_string(),
                    "-H".to_string(),
                ],
            ]),
            man_command: "man".to_string(),
            pager_command: Some("less -R".to_string()),
            cheat_sh_url: DEFAULT_CHEAT_SHEET_PROVIDER.to_string(),
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
        let config = Config::parse(&path)?;
        assert!(config.check_help);
        assert!(config.check_version);
        assert_eq!(config.cheat_sh_url, DEFAULT_CHEAT_SHEET_PROVIDER);
        Ok(())
    }
}
