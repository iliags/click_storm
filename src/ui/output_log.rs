use std::sync::{Arc, Mutex};

use cs_scripting::output_log::OutputLog;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OutputLogPane {
    #[serde(skip)]
    pub output_log: Arc<Mutex<OutputLog>>,
}

impl Default for OutputLogPane {
    fn default() -> Self {
        println!("OutputLogPane::default()");
        Self {
            output_log: Arc::new(Mutex::new(OutputLog::default())),
        }
    }
}

impl OutputLogPane {
    pub fn new(output_log: Arc<Mutex<OutputLog>>) -> Self {
        println!("OutputLogPane::new()");
        Self { output_log }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_source("output_log")
            .show(ui, |ui| {
                let output_log = self.output_log.lock().unwrap();
                let mut text_buffer = output_log.get_log_copy();

                ui.add(
                    egui::TextEdit::multiline(&mut text_buffer)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(6)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .cursor_at_end(true),
                );
            });
    }
}
