use crate::error::{Error, Result};
use std::io::Write;
use std::process::{Command, Stdio};
use ureq::AgentBuilder;

/// Cheat sheet provider URL.
const CHEAT_SHEET_PROVIDER: &str = "https://cheat.sh";

/// User agent for the cheat sheet provider.
///
/// See <https://github.com/chubin/cheat.sh/blob/83bffa587b6c1048cbcc40ea8fa6af675203fd5f/bin/app.py#L76>
const CHEAT_SHEET_USER_AGENT: &str = "fetch";

/// Shows the cheat sheet for the given command.
pub fn show_cheat_sheet<Output: Write>(
    cmd: &str,
    pager: &Option<String>,
    output: &mut Output,
) -> Result<()> {
    let client = AgentBuilder::new()
        .user_agent(CHEAT_SHEET_USER_AGENT)
        .build();
    let cheat_sheet = client
        .get(&format!("{}/{}", CHEAT_SHEET_PROVIDER, cmd))
        .call()
        .map_err(|e| Error::from(Box::new(e)))?
        .into_string()?;
    // Don't use a pager when the topic is not found.
    if let Some(pager) = pager
        .as_ref()
        .filter(|_| !cheat_sheet.starts_with("Unknown topic."))
    {
        let mut process = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", pager])
                .stdin(Stdio::piped())
                .spawn()
        } else {
            Command::new("sh")
                .args(["-c", pager])
                .stdin(Stdio::piped())
                .spawn()
        }?;
        if let Some(stdin) = process.stdin.as_mut() {
            writeln!(stdin, "{}", cheat_sheet)?;
            process.wait()?;
        }
    } else {
        writeln!(output, "{}", cheat_sheet)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_cheat_sheet() -> Result<()> {
        let mut output = Vec::new();
        show_cheat_sheet("ls", &None, &mut output)?;
        let output = String::from_utf8_lossy(&output);
        assert!(output.contains(
            "# To display all files, along with the size (with unit suffixes) and timestamp:"
        ));
        assert!(output.contains(
            "# Long format list with size displayed using human-readable units (KiB, MiB, GiB):"
        ));
        Ok(())
    }
}
