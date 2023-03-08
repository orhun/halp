/// Man page helper.
pub mod man;

/// Cheat sheet helper.
pub mod cheat;

use crate::error::Result;
use crate::helper::docs::cheat::show_cheat_sheet;
use crate::helper::docs::man::show_man_page;
use std::io::Write;

/// Shows documentation/usage help about the given binary.
pub fn get_docs_help<Output: Write>(bin: &str, man_cmd: &str, output: &mut Output) -> Result<()> {
    show_man_page(man_cmd, bin)?;
    show_cheat_sheet(bin, output)?;
    Ok(())
}
