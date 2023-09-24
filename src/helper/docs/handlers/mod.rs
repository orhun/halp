/// The command operation handler.
pub mod command;
/// The fetch operation handler.
pub mod fetch;
/// The file read operation handler.
pub mod file;

use crate::error::Result;
use std::collections::HashMap;

/// The operation handler trait.
pub trait Handler {
    /// Handles the operation.
    fn handle(&self, _: String, args_map: &HashMap<String, String>) -> Result<Option<String>>;
}
