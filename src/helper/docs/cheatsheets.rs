use crate::helper::docs::HelpProvider;
use ureq::{AgentBuilder, Request};

/// The default cheatsheets provider URL.
pub const DEFAULT_CHEATSHEETS_PROVIDER: &str =
    "https://raw.githubusercontent.com/cheat/cheatsheets/master";

/// The `cheatsheets` provider
pub struct Cheatsheets;

impl HelpProvider for Cheatsheets {
    fn url(&self) -> &'static str {
        DEFAULT_CHEATSHEETS_PROVIDER
    }

    fn build_request(&self, cmd: &str, url: &str) -> Request {
        AgentBuilder::new().build().get(&format!("{}/{}", url, cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_fetch_cheatsheets() -> Result<()> {
        let output = Cheatsheets.fetch("ls", &None)?;
        assert!(output.contains(
            r##"# To display everything in <dir>, including hidden files:
ls -a <dir>
"##
        ));
        assert!(output.contains(
            r##"# To display directories only, include hidden:
ls -d .*/ */ <dir>
"##
        ));
        Ok(())
    }
}
