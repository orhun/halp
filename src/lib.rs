//! A CLI tool to get help with CLI tools.

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line arguments.
pub mod args;

/// Error handling implementation.
pub mod error;

use crate::args::Args;
use crate::error::Result;
use std::io::Write;

/// Runs `halp`.
pub fn run<Output: Write>(args: Args, output: &mut Output) -> Result<()> {
    Ok(())
}
