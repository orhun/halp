use ureq::{AgentBuilder, Request};
use crate::helper::docs::HelpProvider;

/// EG page provider URL.
pub const DEFAULT_EG_PAGES_PROVIDER: &str = "https://raw.githubusercontent.com/srsudar/eg/master/eg/examples";

/// The `eg` pages provider
pub struct Eg;

impl HelpProvider for Eg {
    fn url(&self) -> &'static str {
        DEFAULT_EG_PAGES_PROVIDER
    }

    fn build_req(&self, cmd: &str, url: &str) -> Request {
        AgentBuilder::new().build().get(&format!("{}/{}.md", url, cmd))
    }
}
