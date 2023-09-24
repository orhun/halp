use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::helper::docs::handlers::Handler;

/// External command operation handler.
pub struct CommandHandler;

impl Handler for CommandHandler {
    /// Execute an external command.
    ///
    /// # Arguments
    /// - `command_str`: The command to execute.
    /// - `args_map`: The arguments map.
    ///
    /// ## Possible arguments that can be put in the arguments map
    /// - `cwd`: The command working directory, default is the current working directory.
    /// - `env`: The command environment variables in the format `key=value`, default is the current environment variables.
    /// - `timeout`: The command timeout in seconds, default is infinite.
    /// - `use-pager`: Use a pager to display the command output, default is `false`.
    fn handle(
        &self,
        command_str: String,
        args_map: &HashMap<String, String>,
    ) -> Result<Option<String>> {
        let mut command = create_command(&command_str);
        if let Some(cwd) = args_map.get("cwd") {
            command.current_dir(cwd);
        }
        if let Some(env) = args_map.get("env") {
            command.envs(parse_env(env)?);
        }
        execute_command(command, args_map)
    }
}

/// Parse the environment variables string in`key=value,key=value` format.
fn parse_env(env: &str) -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();
    for env in env.split(',') {
        let split = env.split_once('=');
        if let Some((key, value)) = split {
            env_map.insert(key.to_string(), value.to_string());
        } else {
            return Err(Error::InvalidArgument("env".to_string()));
        }
    }
    Ok(env_map)
}

/// Execute the command.
/// if the timeout is 0 then execute the command without a timeout.
macro_rules! execute {
    ($command: ident, $timeout: expr) => {{
        let timeout = $timeout;
        if timeout > 0 {
            spawn_with_timeout($command, $timeout)?
        } else {
            $command.spawn()?
        }
    }};
}

/// Collect the output of the command to string.
macro_rules! collect_command_output {
    ($output: expr) => {{
        let output = String::from_utf8($output.stdout)
            .map_err(|_| Error::CommandError("Failed to read the command output".to_string()))?;
        output
    }};
}

/// Execute the command and pipe the output to another command if needed.
#[inline(always)]
fn execute_command(
    mut command: Command,
    args_map: &HashMap<String, String>,
) -> Result<Option<String>> {
    let use_pager = if let Some(use_pager) = args_map.get("use-pager") {
        if use_pager == "true" || use_pager == "1" {
            // Set the stdout to the pipe configuration.
            command.stdout(Stdio::piped());
            true
        } else {
            false
        }
    } else {
        false
    };
    let mut process = execute!(
        command,
        if let Some(timeout) = args_map.get("timeout") {
            timeout
                .parse::<u64>()
                .map_err(|_| Error::InvalidArgument("timeout".to_string()))?
        } else {
            0
        }
    );
    // If the `use-pager` argument is set to `true` then collect the output to return it later.
    Ok(if use_pager {
        Some(collect_command_output!(process.wait_with_output()?))
    } else {
        process.wait()?;
        None
    })
}

/// Spawn(execute) a command with for a specified time.
///
/// if the timeout is reached the execution will be terminated and an error will be returned.
///
///# Panics
/// This function will panic if the command thread failed to join.
fn spawn_with_timeout(mut command: Command, timeout: u64) -> Result<Child> {
    let execute_thread = thread::spawn(move || command.spawn());
    // Wait for the command for the specified timeout.
    for _ in 0..timeout {
        if execute_thread.is_finished() {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    // If the command is still running, kill it and return an error.
    if !execute_thread.is_finished() {
        return Err(Error::CommandTimeoutError);
    }
    Ok(execute_thread
        .join()
        .expect("Failed to join the command thread.")?)
}

fn create_command(cmd: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(["/C", cmd]);
        command
    } else {
        let mut command = Command::new("sh");
        command.args(["-c", cmd]);
        command
    };
    command.stdin(Stdio::piped());
    command
}
