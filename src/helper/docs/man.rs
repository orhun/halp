use crate::error::Result;
use std::process::Command;

/// Runs the manual page command.
pub fn show_man_page(man_cmd: &str, cmd: &str) -> Result<()> {
    let command = format!("{} {}", man_cmd, cmd);
    let mut process = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &command]).spawn()
    } else {
        Command::new("sh").args(["-c", &command]).spawn()
    }?;
    process.wait()?;
    Ok(())
}
