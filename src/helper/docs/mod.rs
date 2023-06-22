/// Man page helper.
pub mod man;

/// Cheat sheet helper.
pub mod cheat;
/// eg page helper.
mod eg;

use crate::error::Result;
use crate::helper::docs::cheat::show_cheat_sheet;
use crate::helper::docs::man::show_man_page;
use console::{style, Style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Write;
use crate::helper::docs::eg::show_eg_page;

/// Shows documentation/usage help about the given command.
pub fn get_docs_help<Output: Write>(
    cmd: &str,
    man_cmd: &str,
    pager: Option<String>,
    output: &mut Output,
) -> Result<()> {
    let mut selection = Some(0);
    loop {
        selection = Select::with_theme(&get_selection_theme())
            .with_prompt("Select operation")
            .default(selection.unwrap_or_default())
            .items(&["Show man page", "Show cheat sheet", "Show the eg page", "Exit"])
            .interact_on_opt(&Term::stderr())?;
        match selection {
            Some(0) => show_man_page(man_cmd, cmd)?,
            Some(1) => show_cheat_sheet(cmd, &pager, output)?,
            Some(2) => show_eg_page(cmd, &pager, output)?,
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
