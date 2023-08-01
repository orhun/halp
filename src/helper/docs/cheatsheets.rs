use ureq::{AgentBuilder, Request};
use crate::helper::docs::HelpProvider;

pub const DEFAULT_CHEATSHEETS_PROVIDER: &str =
    "https://raw.githubusercontent.com/cheat/cheatsheets/master";

/// The `cheatsheets` provider
pub struct Cheatsheets;

impl HelpProvider for Cheatsheets {
    fn url(&self) -> &'static str {
        DEFAULT_CHEATSHEETS_PROVIDER
    }

    fn build_request(&self, cmd: &str, url: &str) -> Request {
        AgentBuilder::new()
            .build()
            .get(&format!("{}/{}", url, cmd))
    }
}
