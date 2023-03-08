use crate::config::Config;
use crate::error::Result;
use std::io::Write;
use std::process::Command;

/// Shows documentation/usage help about the given binary.
pub fn get_docs_help<Output: Write>(
    _config: Option<Config>,
    bin: &str,
    man_cmd: &str,
    _output: &mut Output,
) -> Result<()> {
    let mut process = Command::new(man_cmd).arg(bin).spawn()?;
    process.wait()?;
    Ok(())
}
