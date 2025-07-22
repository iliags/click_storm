use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Simple script structure
///
/// The `script_buffer` is used as the primary workspace, the script is used to store the last saved
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Script {
    script: String,
    script_buffer: String,
    script_path: Option<PathBuf>,
}

impl Script {
    /// Create a new script
    #[must_use]
    pub fn new() -> Self {
        Self {
            script: String::new(),
            script_buffer: String::new(),
            script_path: None,
        }
    }

    /// Load a script from a file
    pub fn load(&mut self, path: Option<PathBuf>) {
        match path {
            Some(path) => {
                let script = std::fs::read_to_string(&path).unwrap();
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
        self.script_buffer = script.to_string();
        self.script = script.to_string();
    }

    /// Get the script path
    pub fn set_script_path(&mut self, script_path: Option<PathBuf>) {
        self.script_path = script_path;
    }

    /// Check if the script has changes
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.script_buffer != self.script && self.script_path.is_some()
    }

    /// Save the script buffer to the script, returns true if the script is new
    // TODO: Make this more ergonomic
    pub fn save(&mut self) -> bool {
        self.script = self.script_buffer.clone();

        if self.script_path.is_none() {
            true
        } else {
            // TODO: Handle errors
            let path = self.script_path.as_ref().unwrap();
            std::fs::write(path, &self.script_buffer).unwrap();
            false
        }
    }

    /// Get the script
    #[must_use]
    pub fn get_ref(&self) -> &str {
        &self.script_buffer
    }

    /// Get a copy of the script
    #[must_use]
    pub fn get_copy(&self) -> String {
        self.script_buffer.clone()
    }

    /// Get the script (mutable)
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.script_buffer
    }

    /// Get the script path
    #[must_use]
    pub fn get_path(&self) -> Option<PathBuf> {
        self.script_path.clone()
    }

    /// Check if the script is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.script_buffer.is_empty()
    }

    /// Check if the script is the default empty script
    #[must_use]
    pub fn is_default(&self) -> bool {
        self.script_buffer.is_empty() && self.script.is_empty() && self.script_path.is_none()
    }

    /// Get the filename
    #[must_use]
    pub fn get_filename(&self) -> Option<String> {
        self.script_path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .map(std::string::ToString::to_string)
    }

    /// Reload the script from disk
    pub fn reload_from_disk(&mut self) {
        if let Some(path) = self.script_path.clone() {
            let script = std::fs::read_to_string(&path).unwrap();
            self.set_script(&script);
        }
    }

    /// Check if the script has a path
    #[must_use]
    pub fn has_path(&self) -> bool {
        self.script_path.is_some()
    }
}
