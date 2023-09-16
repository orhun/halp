use std::collections::HashMap;
use std::time::Duration;
use ureq::{AgentBuilder, Proxy};

use crate::error::{Error, Result};
use crate::helper::docs::handlers::Handler;

/// Fetch pages from an external source by http.
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use halp::helper::docs::handlers::fetch::FetchHandler;
/// # use halp::error::Result;
/// # use halp::helper::docs::handlers::Handler;
///
/// let git_cheat_page = FetchHandler.handle("https://cheat.sh/git".to_string(), &HashMap::new());
/// assert!(git_cheat_page.is_ok());
/// let git_cheat_page = git_cheat_page.unwrap();
/// assert!(git_cheat_page.is_some());
/// println!("{}", git_cheat_page.unwrap());
/// ```
pub struct FetchHandler;

impl Handler for FetchHandler {
    /// Fetch the help page from an external source by http.
    ///
    /// The first argument is the URL to fetch, the rest of the arguments is used to configure the request.
    /// The first argument is required, the rest is optional.
    ///
    /// The possible arguments are:
    /// - `method`: The HTTP method to use, default is `GET`.
    /// - `body`: The request body, default is empty.
    /// - `headers`: The request headers, default is empty.
    /// - `timeout`: The request timeout in seconds, default is 10 seconds.
    /// - `user-agent`: The request user agent, default is `help me plz - <Halp version>`.
    /// - `proxy`: The request proxy, default is empty.
    fn handle(
        &self,
        op_value: String,
        args_map: &HashMap<String, String>,
    ) -> Result<Option<String>> {
        // build request
        let mut agent_builder = AgentBuilder::new().user_agent(
            args_map
                .get("user-agent")
                .unwrap_or(&format!("help me plz - {}", env!("CARGO_PKG_VERSION")).to_string()),
        );
        if let Some(proxy) = args_map.get("proxy") {
            agent_builder = agent_builder
                .proxy(Proxy::new(proxy).map_err(|_| Error::InvalidArgument("proxy".to_string()))?)
        }
        let agent = agent_builder.build();
        let mut request = agent
            .request(
                args_map.get("method").unwrap_or(&"GET".to_string()),
                &op_value,
            )
            .timeout(if let Some(timeout) = args_map.get("timeout") {
                Duration::from_secs(
                    timeout
                        .parse::<u64>()
                        .map_err(|_| Error::InvalidArgument("timeout".to_string()))?,
                )
            } else {
                Duration::from_secs(10)
            });
        // add headers if any
        if let Some(headers) = args_map.get("headers") {
            for header in headers.split(',') {
                let mut header = header.split(':');
                request = request.set(
                    header.next().unwrap_or("").trim(),
                    header.next().unwrap_or("").trim(),
                );
            }
        }
        let request = if let Some(body) = args_map.get("body") {
            request.send_string(body)
        } else {
            request.call()
        }
        .map_err(|e| Error::from(Box::new(e)))?;
        let response = request
            .into_string()
            .map_err(|e| Error::ProviderError(e.to_string()))?;
        // handle potential errors
        if response.is_empty()
            || response.contains("Unknown topic")
            || response.contains("No manual entry")
        {
            return Err(Error::ProviderError(
                "Unknown topic, This topic/command might has no page in this provider yet."
                    .to_string(),
            ));
        }
        Ok(Some(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fetch_cheat_sheet() -> Result<()> {
        let output = FetchHandler.handle(
            "https://cheat.sh/ls".to_string(),
            &HashMap::from([("user-agent".to_string(), "fetch".to_string())]),
        )?;
        let output = output.expect("output is empty");
        assert!(output.contains(
            "# To display all files, along with the size (with unit suffixes) and timestamp:"
        ));
        assert!(output.contains(
            "# Long format list with size displayed using human-readable units (KiB, MiB, GiB):"
        ));
        Ok(())
    }

    #[test]
    fn test_fetch_unknown_topic() {
        let output = FetchHandler.handle(
            "https://cheat.sh/unknown".to_string(),
            &HashMap::from([("user-agent".to_string(), "fetch".to_string())]),
        );
        assert!(output.is_err());
        assert_eq!(output.expect_err("Unreachable").to_string(),
                   "External help provider error: `Unknown topic, This topic/command might has no page in this provider yet.`");
    }
}
