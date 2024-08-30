#![allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct OutputLog {
    log: String,
}

impl OutputLog {
    pub fn new() -> Self {
        Self { log: String::new() }
    }

    pub fn log(&mut self, message: &str) {
        self.log.push_str(message);
        self.log.push('\n');
    }

    pub fn clear(&mut self) {
        self.log.clear();
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    pub fn get_log_copy(&self) -> String {
        self.log.clone()
    }
}
