use crate::cli::CliArgs;
use crate::config::Config;
use crate::error::Result;
use std::io::Write;

/// Shows documentation/usage help about the given binary.
pub fn get_docs_help<Output: Write>(
    _config: Option<Config>,
    _bin: &str,
    _cli_args: &CliArgs,
    _output: &mut Output,
) -> Result<()> {
    todo!()
}
