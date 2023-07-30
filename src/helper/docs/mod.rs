/// Man page helper.
pub mod man;

/// Cheat sheet helper.
pub mod cheat_sh;
/// eg page helper.
mod eg;

use crate::config::Config;
use crate::error::Result;
use crate::helper::docs::cheat_sh::show_cheat_sheet;
use crate::helper::docs::man::show_man_page;
use console::{style, Style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use ureq::{AgentBuilder, Request};
use crate::helper::docs::eg::show_eg_page;

trait HelpProvider {
    /// Return the default provider URL
    fn url(&self) -> &'static str;

    /// Build the request
    ///
    fn build_req(&self, cmd: &str, url: &str) -> Request {
         AgentBuilder::new().build().get(&format!("{}/{}.md", url, cmd))
    }

    fn fetch(&self, cmd: &str, custom_url: Option<&str>, output: &mut Output) -> Result<()> {
        let response = self.build_req(cmd, custom_url.unwrap_or(self.url())).call();

        let page = match response {
            Ok(page) => page.into_string()?,
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
            .filter(|_| !page.starts_with("Unknown topic."))
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
                writeln!(stdin, "{}", page)?;
                process.wait()?;
            }
        } else {
            writeln!(output, "{}", page)?;
        }

        Ok(())
    }
}

/// Shows documentation/usage help about the given command.
pub fn get_docs_help<Output: Write>(cmd: &str, config: &Config, output: &mut Output) -> Result<()> {
    let mut selection = Some(0);
    loop {
        selection = Select::with_theme(&get_selection_theme())
            .with_prompt("Select operation")
            .default(selection.unwrap_or_default())
            .items(&["Show man page", "Show cheat sheet", "Show the eg page", "Exit"])
            .interact_on_opt(&Term::stderr())?;
        match selection {
            Some(0) => show_man_page(&config.man_command, cmd)?,
            Some(1) => show_cheat_sheet(cmd, &config.cheat_sh_url, &config.pager_command, output)?,
            Some(2) => show_eg_page(cmd, &config.pager_command, output)?,
            _ => return Ok(()),
        };
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
        inline_selections: true,
    }
}
