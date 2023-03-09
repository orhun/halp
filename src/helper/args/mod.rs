/// Common command-line arguments.
pub mod common;

use crate::cli::CliArgs;
use crate::config::Config;
use crate::error::Result;
use crate::helper::args::common::{HelpArg, VersionArg};
use colored::*;
use std::io::Write;
use std::process::Command;
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
            CHECK_EMOTICON.magenta(),
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

/// Shows command-line help about the given binary.
pub fn get_args_help<Output: Write>(
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
}
