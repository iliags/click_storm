#![allow(dead_code)]

/// Simple output log
#[derive(Default, Debug, Clone)]
pub struct OutputLog {
    log: String,
}

impl OutputLog {
    /// Create a new output log
    pub fn new() -> Self {
        Self { log: String::new() }
    }

    /// Store a message in the log
    pub fn log(&mut self, message: &str) {
        self.log.push_str(message);
        self.log.push('\n');
    }

    /// Clear the log
    pub fn clear(&mut self) {
        self.log.clear();
    }

    /// Get the log
    pub fn get_log(&self) -> &str {
        &self.log
    }

    /// Get a copy of the log
    pub fn get_log_copy(&self) -> String {
        self.log.clone()
    }
}
