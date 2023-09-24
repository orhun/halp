/// Man page helper.
pub mod man;

/// Cheat sheet helper.
pub mod cheat_sh;
/// cheat helper.
pub mod cheatsheets;
/// eg page helper.
pub mod eg;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::helper::docs::cheat_sh::CheatDotSh;
use crate::helper::docs::cheatsheets::Cheatsheets;
use crate::helper::docs::eg::Eg;
use crate::helper::docs::man::show_man_page;
use console::{style, Style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Write;
use std::process::{Command, Stdio};
use ureq::Request;

/// The `HelpProvider` trait defines essential methods for fetching help content related to commands from a provider.
///
/// Each provider that implements this trait should provide a default URL used to retrieve the command help content.
/// However, it also allows specifying a custom URL to override the default one.
///
/// This trait is generic and not tied to any specific command help system such as man pages or POSIX documentation,
/// instead it relies on the implementation to define how to fetch and mark up the content.
///
/// # Methods
///
/// - `url`: Returns the default URL of the provider.
/// - `build_req`: Uses the given command and URL to create an HTTP GET request.
/// - `err_handle`: Handles possible request errors, such as a non-existent command page on the provider.
/// - `fetch`: Attempts to retrieve the command page from the provider, optionally from a custom URL.
///
/// # Example
///
/// An implementation could be created for a provider that supplies help pages in Markdown format.
/// The `url` method would return the base URL of this provider.
/// The `build_request` method could construct a GET request for `{base_url}/{command}.md`.
/// The `handle_error` could interpret a 404 status code as 'Command page not found'.
/// The `fetch` would handle fetching the specified command page using the constructed request.
pub trait HelpProvider {
    /// Return the default provider URL.
    fn url(&self) -> &'static str;

    /// Builds an HTTP request using the given `cmd` and `url`.
    ///
    /// # Parameters
    ///
    /// - `cmd`: The name of the command to be included in the request.
    /// - `url`: The root URL.
    ///
    /// # Returns
    /// This method returns a new `Request` instance configured with the `GET` method and the formatted URL.
    fn build_request(&self, cmd: &str, url: &str) -> Request;

    /// Handle the request error.
    /// aka return a custom message if the error means that **provider** doesn't have a page for the command.
    fn handle_error(&self, e: ureq::Error) -> Error {
        if e.kind() == ureq::ErrorKind::HTTP {
            Error::ProviderError(
                "Unknown topic, This topic/command might has no page in this provider yet."
                    .to_string(),
            )
        } else {
            Error::from(Box::new(e))
        }
    }

    /// **The default** fetch implementation.
    ///
    /// This method attempts to retrieve the specified command page from the given provider.
    /// If a `custom_url` is provided, this URL is used instead of the default URL.
    /// The method will return the content of the command page if the fetch operation is successful.
    #[inline(always)]
    fn _fetch(&self, cmd: &str, custom_url: &Option<String>) -> Result<String> {
        let url = {
            if let Some(u) = custom_url {
                u.as_str()
            } else {
                self.url()
            }
        };
        let response = self.build_request(cmd, url).call();

        let response = response.map_err(|e| self.handle_error(e));

        match response {
            Ok(response) => Ok(response.into_string()?),
            Err(e) => Err(e),
        }
    }

    /// Fetches the command page from the provider.
    ///
    /// # Parameters
    ///
    /// - `cmd`: The name of the command for which the page should be fetched.
    /// - `custom_url`: Optional parameter that, if supplied, specifies a custom URL from which to fetch the command page.
    ///
    /// # Returns
    ///
    /// This method returns a Result type. On successful fetch, it contains a `String` with the content of the fetched page.
    /// In case of failure, it contains an error that provides further details about the issue encountered during the fetch operation.
    ///
    /// # Errors
    ///
    /// This method will return an error if the fetch operation fails.
    fn fetch(&self, cmd: &str, custom_url: &Option<String>) -> Result<String> {
        self._fetch(cmd, custom_url)
    }
}

/// Shows documentation/usage help about the given command.
pub fn get_docs_help<Output: Write>(cmd: &str, config: &Config, output: &mut Output) -> Result<()> {
    const MAN_PAGE: usize = 0;
    const CHEAT_SHEET: usize = 1;
    const EG_PAGE: usize = 2;
    const CHEATSHEETS: usize = 3;
    let menu_options = [
        "Show man page",
        "Show cheat.sh page",
        "Show the eg page",
        "Show the cheatsheet page",
        "Exit",
    ];
    let mut selection = Some(MAN_PAGE);
    loop {
        selection = Select::with_theme(&get_selection_theme())
            .with_prompt("Select operation")
            .default(selection.unwrap_or_default())
            .items(&menu_options)
            .interact_on_opt(&Term::stderr())?;
        if let Some(MAN_PAGE) = selection {
            show_man_page(&config.man_command, cmd)?
        } else {
            let page = match selection {
                Some(CHEAT_SHEET) => CheatDotSh.fetch(cmd, &config.cheat_sh_url)?,
                Some(EG_PAGE) => Eg.fetch(cmd, &config.eg_url)?,
                Some(CHEATSHEETS) => Cheatsheets.fetch(cmd, &config.cheatsheets_url)?,
                _ => return Ok(()),
            };
            // Show the page using the user selected pager or write it directly into the output
            if let Some(pager) = config.pager_command.as_ref() {
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
                    writeln!(stdin, "{}", page)?;
                    process.wait()?;
                }
            } else {
                writeln!(output, "{}", page)?;
            }
        }
    }
}

/// Returns the theme for selection prompt.
fn get_selection_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().cyan(),
        prompt_style: Style::new().for_stderr().bold(),
        prompt_prefix: style("(ﾉ´ヮ`)ﾉ*: ･ﾟ\n".to_string()).for_stderr().magenta(),
        prompt_suffix: style("›".to_string()).for_stderr().magenta().bright(),
        success_prefix: style("❤".to_string()).for_stderr().magenta(),
        success_suffix: style("·".to_string()).for_stderr().black().bright(),
        error_prefix: style("✘".to_string()).for_stderr().red(),
        error_style: Style::new().for_stderr().red(),
        hint_style: Style::new().for_stderr().black().bright(),
        values_style: Style::new().for_stderr().green(),
        active_item_style: Style::new().for_stderr().green().bold(),
        inactive_item_style: Style::new().for_stderr().italic(),
        active_item_prefix: style("✧".to_string()).for_stderr().magenta().bold(),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("❤".to_string()).for_stderr().magenta(),
        unchecked_item_prefix: style("❤".to_string()).for_stderr().black(),
        picked_item_prefix: style("❯".to_string()).for_stderr().green(),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
    }
}
