use crate::error::{Error, Result};
use std::io::Write;
use ureq::AgentBuilder;

/// Cheat sheet provider URL.
const CHEAT_SHEET_PROVIDER: &str = "https://cheat.sh";

/// User agent for the cheat sheet provider.
///
/// See <https://github.com/chubin/cheat.sh/blob/83bffa587b6c1048cbcc40ea8fa6af675203fd5f/bin/app.py#L76>
const CHEAT_SHEET_USER_AGENT: &str = "fetch";

/// Shows the cheat sheet for the given binary.
pub fn show_cheat_sheet<Output: Write>(bin: &str, output: &mut Output) -> Result<()> {
    let client = AgentBuilder::new()
        .user_agent(CHEAT_SHEET_USER_AGENT)
        .build();
    let cheat_sheet = client
        .get(&format!("{}/{}", CHEAT_SHEET_PROVIDER, bin))
        .call()
        .map_err(|e| Error::from(Box::new(e)))?
        .into_string()?;
    writeln!(output, "{}", cheat_sheet)?;
    Ok(())
}
