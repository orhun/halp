/// Man page helper.
pub mod man;

/// Cheat sheet helper.
pub mod cheat;

use crate::error::Result;
use crate::helper::docs::cheat::show_cheat_sheet;
use crate::helper::docs::man::show_man_page;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io::Write;

/// Shows documentation/usage help about the given binary.
pub fn get_docs_help<Output: Write>(bin: &str, man_cmd: &str, output: &mut Output) -> Result<()> {
    let mut selection = Some(0);
    loop {
        selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select operation")
            .default(selection.unwrap_or_default())
            .items(&["Show man page", "Show cheat sheet", "Exit"])
            .interact_on_opt(&Term::stderr())?;
        match selection {
            Some(0) => show_man_page(man_cmd, bin)?,
            Some(1) => show_cheat_sheet(bin, output)?,
            _ => return Ok(()),
        };
    }
}
