use crate::localization::locale_text::LocaleText;

use super::UIPanel;

pub const SCRIPT_PANEL_KEY: &str = "script_panel";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    #[serde(skip)]
    language: LocaleText,
}

impl Default for ScriptPanel {
    fn default() -> Self {
        Self {
            language: LocaleText::default(),
        }
    }
}

impl UIPanel for ScriptPanel {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("Script Panel");
    }

    fn start(&mut self) {
        println!("Starting script");
    }

    fn stop(&mut self) {
        println!("Stopping script");
    }

    fn toggle(&mut self) {
        println!("Toggling script");
    }

    fn is_running(&self) -> bool {
        false
    }

    fn handle_input(&mut self) {}

    fn reset(&mut self) {}

    fn exit(&mut self) {}

    fn set_language(&mut self, language: LocaleText) {
        self.language = language;
    }

    fn name(&self) -> &str {
        // TODO: Get the localized string
        "Script"
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ScriptPanel {
    pub fn load(&mut self, _value: ScriptPanel) {
        // TODO
    }
}
