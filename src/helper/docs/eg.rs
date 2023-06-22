use std::io::Write;
use std::process::{Command, Stdio};
use ureq::AgentBuilder;
use crate::error::{Result, Error};

/// EG page provider URL.
const EG_PAGES_PROVIDER: &str = "https://raw.githubusercontent.com/srsudar/eg/master/eg/examples";

/// Shows the EG page for the given command.
pub fn show_eg_page<Output: Write>(
    cmd: &str,
    pager: &Option<String>,
    output: &mut Output,
) -> Result<()> {
    let client = AgentBuilder::new().build();
    let eg_page = client
        .get(&format!("{}/{}.md", EG_PAGES_PROVIDER, cmd))
        .call();

    let eg_page = match eg_page {
        Ok(eg_page) => eg_page.into_string()?,
        Err(e) => {
            if e.kind() == ureq::ErrorKind::HTTP {
                "Unknown topic.\nThis topic/command has no eg page yet.".to_string()
            } else {
                return Err(Error::from(Box::new(e)));
            }
        }
    };

    // Don't use a pager when the topic is not found.
    if let Some(pager) = pager
        .as_ref()
        .filter(|_| !eg_page.starts_with("Unknown topic."))
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
            writeln!(stdin, "{}", eg_page)?;
            process.wait()?;
        }
    } else {
        writeln!(output, "{}", eg_page)?;
    }

    Ok(())
}
