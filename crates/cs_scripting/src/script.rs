use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Simple script structure
///
/// The `buffer` is used as the primary workspace, `last_saved` is used to store the last saved buffer
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Script {
    last_saved: String,
    buffer: String,
    path: Option<PathBuf>,
}

impl Script {
    /// Create a new script
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_saved: String::new(),
            buffer: String::new(),
            path: None,
        }
    }

    /// Load a script from a file
    pub fn load(&mut self, path: Option<PathBuf>) {
        match path {
            Some(path) => {
                let script = std::fs::read_to_string(&path).unwrap_or_default();
                self.set_script(&script);
                self.set_script_path(Some(path));
            }
            None => {
                eprintln!("No path provided to load script from");
            }
        }
    }

    /// Set the script
    pub fn set_script(&mut self, script: &str) {
        self.buffer = script.to_string();
        self.last_saved = script.to_string();
    }

    /// Get the script path
    pub fn set_script_path(&mut self, script_path: Option<PathBuf>) {
        self.path = script_path;
    }

    /// Check if the script has changes
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.buffer != self.last_saved && self.path.is_some()
    }

    /// Save the buffer to the in-memory save, returns true if the script is new
    // TODO: Make this more ergonomic
    pub fn save(&mut self) -> bool {
        self.last_saved = self.buffer.clone();

        if self.path.is_none() {
            true
        } else {
            // TODO: Handle errors
            match self.path.as_ref() {
                Some(path) => {
                    let _ = std::fs::write(path, &self.buffer);
                }
                None => eprintln!("Invalid path given"),
            }
            false
        }
    }

    /// Get the script
    #[must_use]
    pub fn get_ref(&self) -> &str {
        &self.buffer
    }

    /// Get a copy of the script
    #[must_use]
    pub fn get_copy(&self) -> String {
        self.buffer.clone()
    }

    /// Get the script (mutable)
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.buffer
    }

    /// Get the script path
    #[must_use]
    pub fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
    }

    /// Check if the script is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if the script is the default empty script
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.buffer.is_empty() && self.last_saved.is_empty() && self.path.is_none()
    }

    /// Get the filename
    #[must_use]
    pub fn get_filename(&self) -> Option<String> {
        self.path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .map(std::string::ToString::to_string)
    }

    /// Reload the script from disk
    pub fn reload_from_disk(&mut self) {
        if let Some(path) = self.path.clone() {
            let script = std::fs::read_to_string(&path).unwrap_or_default();
            self.set_script(&script);
        }
    }

    /// Check if the script has a path
    #[must_use]
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }
}
