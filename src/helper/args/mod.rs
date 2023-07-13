/// Helper module for Help and Version checks variants.
pub mod common;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::helper::tty::TtyCommand;
use colored::*;
use std::io::Write;
use std::process::Stdio;

/// Emoticon for "checking" message.
const CHECK_EMOTICON: &str = "(°ロ°)";
/// Emoticon for "found" message.
pub const FOUND_EMOTICON: &str = "\\(^ヮ^)/";
/// Emoticon for "fail" message.
pub const FAIL_EMOTICON: &str = "(×﹏×)";
/// Emoticon for debug messages.
const DEBUG_EMOTICON: &str = "(o_O)";
/// Separator for output.
const OUTPUT_SEPARATOR: &str = "---";

/// Checks if the given argument exist.
fn check_arg<Output: Write>(
    cmd: &str,
    arg: &str,
    verbose: bool,
    output: &mut Output,
) -> Result<()> {
        let command = format!("{} {}", cmd, arg);
        writeln!(
            output,
            "{}  {} '{}'",
            CHECK_EMOTICON.magenta(),
            "checking".green().bold(),
            command.white().italic()
        )?;
        let cmd_out = TtyCommand::new(&command)?
            .env("PAGER", "")
            .stderr(Stdio::inherit())
            .output()?;
        if cmd_out.status.success() {
            writeln!(
                output,
                "{} {} '{}' argument found!",
                FOUND_EMOTICON.magenta(),
                "success".cyan().bold(),
                arg.white().italic()
            )?;
            writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
            output.write_all(&cmd_out.stdout)?;
            writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
            return Ok(());
        } else {
            writeln!(
                output,
                "{}      {} '{}' argument not found.",
                FAIL_EMOTICON.magenta(),
                "fail".red().bold(),
                arg.white().italic()
            )?;
            if verbose {
                writeln!(
                    output,
                    "{}      {}",
                    DEBUG_EMOTICON.magenta(),
                    "debug".yellow().bold(),
                )?;
                if !cmd_out.stdout.is_empty() {
                    writeln!(output, "{}:", "stdout".white().italic())?;
                    writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
                    output.write_all(&cmd_out.stdout)?;
                    writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
                }
                if !cmd_out.stderr.is_empty() {
                    writeln!(output, "{}:", "stderr".white().italic())?;
                    writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
                    output.write_all(&cmd_out.stderr)?;
                    writeln!(output, "{}", OUTPUT_SEPARATOR.bright_black())?;
                }
            }
        }
    Err(Error::ArgumentNotFoundError)
}

/// Shows command-line help about the given command.
pub fn get_args_help<Output: Write>(
    cmd: &str,
    config: &Config,
    verbose: bool,
    output: &mut Output,
) -> Result<()> {
    if cmd.trim().is_empty() {
        return Ok(());
    }
    if let Some(ref args) = config.check_args {
        if args.is_empty() {
            return Ok(());
        }
        for arg_variants in
            if args[0].is_empty() {
                [(args.len() > 1).then(|| &args[1..]), None]

            } else {
                let x = [
                    (config.check_version).then(|| &args[..1]),
                    (config.check_help && args.len() >= 2).then(|| &args[1..]),
                ];
                x
            }
        .iter()
        .flatten()
        {
            if args[0].is_empty() {
                // If the user uses the `--check` option then we check all the arguments.
                arg_variants.iter().flatten().for_each(|arg| {
                    let _ = check_arg(cmd, arg, verbose, output);
                });

            } else {
                // otherwise we check only the first argument found.
                for arg in arg_variants.iter().flatten() {
                    if let Ok(()) = check_arg(cmd, arg, verbose, output) {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::args::common::{HelpArg, VersionArg};
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    /// Returns the path of the test binary.
    fn get_test_bin() -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(format!("{}-test", env!("CARGO_PKG_NAME")))
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn test_check_version_args() -> Result<()> {
        let mut output = Vec::new();
        check_args(
            &get_test_bin(),
            VersionArg::variants().iter().map(|v| v.as_str()),
            false,
            &mut output,
        )?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -v'
(×﹏×)      fail '-v' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
---
halp 0.1.0
---"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .replace(env!("CARGO_PKG_VERSION"), "0.1.0")
                .trim()
        );

        Ok(())
    }

    #[test]
    fn test_check_help_args() -> Result<()> {
        let mut output = Vec::new();
        check_args(
            &get_test_bin(),
            HelpArg::variants().iter().rev().map(|v| v.as_str()),
            true,
            &mut output,
        )?;
        assert_eq!(
            r#"(°ロ°)  checking 'test -H'
(×﹏×)      fail '-H' argument not found.
(o_O)      debug
stdout:
---
error: unexpected argument '-H' found

Usage: test

For more information, try '--help'.
---
(°ロ°)  checking 'test help'
(×﹏×)      fail 'help' argument not found.
(o_O)      debug
stdout:
---
error: unexpected argument 'help' found

Usage: test

For more information, try '--help'.
---
(°ロ°)  checking 'test --help'
\(^ヮ^)/ success '--help' argument found!
---
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version
---"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .trim()
        );

        Ok(())
    }

    #[test]
    fn test_get_default_help() -> Result<()> {
        let config = Config::default();
        let mut output = Vec::new();
        get_args_help(&get_test_bin(), &config, false, &mut output)?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -v'
(×﹏×)      fail '-v' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
---
halp 0.1.0
---
(°ロ°)  checking 'test -h'
\(^ヮ^)/ success '-h' argument found!
---
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version
---"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .replace(env!("CARGO_PKG_VERSION"), "0.1.0")
                .trim()
        );
        Ok(())
    }

    #[test]
    fn test_get_args_help() -> Result<()> {
        let config = Config {
            check_args: Some(vec![vec![String::from("-x")], vec![String::from("-V")]]),
            ..Default::default()
        };
        let mut output = Vec::new();
        get_args_help(&get_test_bin(), &config, false, &mut output)?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -x'
(×﹏×)      fail '-x' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
---
halp 0.1.0
---"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .replace(env!("CARGO_PKG_VERSION"), "0.1.0")
                .trim()
        );
        Ok(())
    }

    #[test]
    fn test_do_nothing() -> Result<()> {
        let config = Config {
            check_version: false,
            check_help: false,
            ..Default::default()
        };
        let mut output = Vec::new();
        get_args_help("", &config, false, &mut output)?;
        assert!(String::from_utf8_lossy(&output).is_empty());
        Ok(())
    }
}
