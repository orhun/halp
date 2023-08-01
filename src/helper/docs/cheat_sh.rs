use crate::error::{Error, Result};
use crate::helper::docs::HelpProvider;
use ureq::{AgentBuilder, Request};

/// Default cheat sheet provider URL.
pub const DEFAULT_CHEAT_SHEET_PROVIDER: &str = "https://cheat.sh";

/// User agent for the cheat sheet provider.
///
/// See <https://github.com/chubin/cheat.sh/blob/83bffa587b6c1048cbcc40ea8fa6af675203fd5f/bin/app.py#L76>
const CHEAT_SHEET_USER_AGENT: &str = "fetch";

/// The `cheat.sh` provider
pub struct CheatDotSh;

impl HelpProvider for CheatDotSh {
    fn url(&self) -> &'static str {
        DEFAULT_CHEAT_SHEET_PROVIDER
    }

    fn build_request(&self, cmd: &str, url: &str) -> Request {
        AgentBuilder::new()
            .user_agent(CHEAT_SHEET_USER_AGENT)
            .build()
            .get(&format!("{}/{}", url, cmd))
    }

    fn fetch(&self, cmd: &str, custom_url: &Option<String>) -> Result<String> {
        let response = self._fetch(cmd, custom_url);
        if let Ok(page) = &response {
            if page.starts_with("Unknown topic.") {
                return Err(Error::ProviderError(page.to_owned()));
            }
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_cheat_sheet() -> Result<()> {
        let output = CheatDotSh.fetch("ls", &None)?;
        assert!(output.contains(
            "# To display all files, along with the size (with unit suffixes) and timestamp:"
        ));
        assert!(output.contains(
            "# Long format list with size displayed using human-readable units (KiB, MiB, GiB):"
        ));
        Ok(())
    }
}
