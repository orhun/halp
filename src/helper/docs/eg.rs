use crate::helper::docs::HelpProvider;
use ureq::{AgentBuilder, Request};

/// EG page provider URL.
pub const DEFAULT_EG_PAGES_PROVIDER: &str =
    "https://raw.githubusercontent.com/srsudar/eg/master/eg/examples";

/// The `eg` pages provider
pub struct Eg;

impl HelpProvider for Eg {
    fn url(&self) -> &'static str {
        DEFAULT_EG_PAGES_PROVIDER
    }

    fn build_request(&self, cmd: &str, url: &str) -> Request {
        AgentBuilder::new()
            .build()
            .get(&format!("{}/{}.md", url, cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_eg_page_fetch() -> Result<()> {
        let output = Eg.fetch("ls", &None)?;
        assert!(output.contains("show contents of current directory"));
        assert!(output.contains("ls -alh"));
        assert!(output.contains(
            r#"`ls` is often aliased to make the defaults a bit more useful. Here are three
basic aliases. The second two can be remembered by "list long" and "list all":
"#
        ));
        Ok(())
    }
}
