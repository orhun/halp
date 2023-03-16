/// Common command-line arguments.
pub mod common;

use crate::cli::CliArgs;
use crate::config::Config;
use crate::error::Result;
use crate::helper::args::common::{HelpArg, VersionArg};
use crate::helper::tty::TtyCommand;
use colored::*;
use std::io::Write;
use std::process::Stdio;

/// Emoticon for "checking" message.
const CHECK_EMOTICON: &str = "(°ロ°)";
/// Emoticon for "found" message.
const FOUND_EMOTICON: &str = "\\(^ヮ^)/";
/// Emoticon for "fail" message.
const FAIL_EMOTICON: &str = "(×﹏×)";
/// Emoticon for debug messages.
const DEBUG_EMOTICON: &str = "(o_O)";

/// Checks if the given arguments exist.
fn check_args<'a, ArgsIter: Iterator<Item = &'a str>, Output: Write>(
    cmd: &str,
    args: ArgsIter,
    verbose: bool,
    output: &mut Output,
) -> Result<()> {
    for arg in args {
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
            output.write_all(&cmd_out.stdout)?;
            break;
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

/// Shows command-line help about the given command.
pub fn get_args_help<Output: Write>(
    cmd: &str,
    cli_args: &CliArgs,
    config: Option<Config>,
    output: &mut Output,
) -> Result<()> {
    if let Some(config_args) = config.and_then(|v| v.check_args) {
        for args in config_args {
            check_args(
                cmd,
                args.iter().map(|v| v.as_str()),
                cli_args.verbose,
                output,
            )?;
        }
        return Ok(());
    }
    if let Some(ref args) = cli_args.check_args {
        check_args(
            cmd,
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
            cmd,
            arg_variants.iter().map(|v| v.as_str()),
            cli_args.verbose,
            output,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
halp 0.1.0"#,
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
error: unexpected argument '-H' found

Usage: test

For more information, try '--help'.
(°ロ°)  checking 'test help'
(×﹏×)      fail 'help' argument not found.
(o_O)      debug
stdout:
error: unexpected argument 'help' found

Usage: test

For more information, try '--help'.
(°ロ°)  checking 'test --help'
\(^ヮ^)/ success '--help' argument found!
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .trim()
        );

        Ok(())
    }

    #[test]
    fn test_get_default_help() -> Result<()> {
        let args = CliArgs::default();
        let mut output = Vec::new();
        get_args_help(&get_test_bin(), &args, None, &mut output)?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -v'
(×﹏×)      fail '-v' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
halp 0.1.0
(°ロ°)  checking 'test -h'
\(^ヮ^)/ success '-h' argument found!
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version"#,
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
        let args = CliArgs {
            check_args: Some(vec![String::from("-x"), String::from("-V")]),
            ..Default::default()
        };
        let mut output = Vec::new();
        get_args_help(&get_test_bin(), &args, None, &mut output)?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -x'
(×﹏×)      fail '-x' argument not found.
(°ロ°)  checking 'test -V'
\(^ヮ^)/ success '-V' argument found!
halp 0.1.0"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .replace(env!("CARGO_PKG_VERSION"), "0.1.0")
                .trim()
        );
        Ok(())
    }

    #[test]
    fn test_get_config_help() -> Result<()> {
        let args = CliArgs::default();
        let config = Config {
            check_args: Some(vec![vec![String::from("-y"), String::from("--help")]]),
            ..Default::default()
        };
        let mut output = Vec::new();
        get_args_help(&get_test_bin(), &args, Some(config), &mut output)?;
        println!("{}", String::from_utf8_lossy(&output));
        assert_eq!(
            r#"(°ロ°)  checking 'test -y'
(×﹏×)      fail '-y' argument not found.
(°ロ°)  checking 'test --help'
\(^ヮ^)/ success '--help' argument found!
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version"#,
            String::from_utf8_lossy(&output)
                .replace('\r', "")
                .replace(&get_test_bin(), "test")
                .trim()
        );
        Ok(())
    }

    #[test]
    fn test_do_nothing() -> Result<()> {
        let args = CliArgs {
            no_version: true,
            no_help: true,
            ..Default::default()
        };
        let mut output = Vec::new();
        get_args_help("", &args, None, &mut output)?;
        assert!(String::from_utf8_lossy(&output).is_empty());
        Ok(())
    }
}
