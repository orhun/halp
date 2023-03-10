use crate::error::{Error, Result};
use std::process::Command as OsCommand;

/// Default shell to use.
const DEFAULT_SHELL: &str = "sh";

/// Command wrapper.
#[derive(Debug)]
pub struct TtyCommand;

impl TtyCommand {
    /// Creates a command with the default shell.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(command: &str) -> Result<OsCommand> {
        Self::new_with_shell(command, None)
    }

    /// Creates a command that is executed by a shell, pretending to be a TTY.
    ///
    /// This means that the command will assume that terminal colors and
    /// other terminal features are available.
    ///
    /// On Linux, the command is wrapped in the `script` command.
    ///
    /// - [Linux docs](https://man7.org/linux/man-pages/man1/script.1.html)
    /// - [FreeBSD docs](https://www.freebsd.org/cgi/man.cgi?query=script&sektion=0&manpath=FreeBSD+12.2-RELEASE+and+Ports&arch=default&format=html)
    /// - [Apple docs](https://opensource.apple.com/source/shell_cmds/shell_cmds-170/script/script.1.auto.html)
    ///
    /// On Windows (which is unsupported), the command is returned as-is.
    pub fn new_with_shell(cmd: &str, shell: Option<&str>) -> Result<OsCommand> {
        if cfg!(any(target_os = "linux", target_os = "android")) {
            let mut command = OsCommand::new("script");
            command.args(["-q", "-e", "-c", cmd, "/dev/null"]);
            if let Some(shell) = shell {
                command.env("SHELL", shell.trim());
            }
            Ok(command)
        } else if cfg!(any(target_os = "macos", target_os = "freebsd")) {
            let mut command = OsCommand::new("script");
            command.args([
                "-q",
                "/dev/null",
                shell.unwrap_or(DEFAULT_SHELL).trim(),
                "-c",
                cmd,
            ]);
            Ok(command)
        } else if cfg!(target_os = "windows") {
            let mut command = OsCommand::new("cmd");
            command.args(["/C", cmd]);
            Ok(command)
        } else {
            Err(Error::UnsupportedPlatformError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::process::Stdio;

    fn run_command(cmd: &str, shell: Option<&str>) -> Result<String> {
        let command = TtyCommand::new_with_shell(cmd, shell)?
            .stderr(Stdio::inherit())
            .output()?;
        Ok(String::from_utf8_lossy(&command.stdout).replace("\r\n", "\n"))
    }

    #[test]
    fn run_echo() -> Result<()> {
        assert_eq!("hello world\n", run_command("echo hello world", None)?);
        Ok(())
    }

    #[test]
    fn run_seq() -> Result<()> {
        assert_eq!("1\n2\n3\n", run_command("seq 3", None)?);
        Ok(())
    }

    #[test]
    fn run_echo_quotes() -> Result<()> {
        assert_eq!(
            "Hello $`' world!\n",
            run_command(r#"echo "Hello \$\`' world!""#, None)?
        );
        Ok(())
    }
}
