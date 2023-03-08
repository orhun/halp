use crate::error::Result;
use std::process::Command;

/// Runs the manual page command.
pub fn show_man_page(man_cmd: &str, bin: &str) -> Result<()> {
    let mut process = Command::new(man_cmd).arg(bin).spawn()?;
    process.wait()?;
    Ok(())
}
