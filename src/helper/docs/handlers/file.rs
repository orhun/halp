use crate::error::Result;
use crate::helper::docs::handlers::Handler;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The file read operation handler.
pub struct FileHandler;

impl Handler for FileHandler {
    /// Just read the file and return its content.
    ///
    /// This operation handler does not support any arguments. i can't think of any useful arguments for this operation :P
    fn handle(&self, path: String, _: &HashMap<String, String>) -> Result<Option<String>> {
        let path = Path::new(&path);
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }
}
