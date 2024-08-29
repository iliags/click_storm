use cs_hal::input::keycode::AppKeycode;
use cs_scripting::rhai_interface::RhaiInterface;
use device_query::DeviceQuery;

use crate::localization::locale_text::LocaleText;

use super::UIPanel;

pub const SCRIPT_PANEL_KEY: &str = "script_panel";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    rhai_interface: RhaiInterface,

    // TODO: Debug only
    #[serde(skip)]
    device_state: device_query::DeviceState,

    #[serde(skip)]
    debug_key: bool,
}

impl Default for ScriptPanel {
    fn default() -> Self {
        let mut rhai_interface = RhaiInterface::new();
        rhai_interface.initialize();

        Self {
            language: LocaleText::default(),
            rhai_interface,

            // TODO: Debug only
            device_state: device_query::DeviceState::new(),
            debug_key: false,
        }
    }
}

impl UIPanel for ScriptPanel {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("Script Panel");

        // Test buttons for development
        #[cfg(debug_assertions)]
        {
            if ui.button("Test Print").clicked() {
                self.rhai_interface.test_hello();
            }

            if ui.button("Test Script").clicked() {
                self.rhai_interface.test_script();
            }
        }
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

    fn handle_input(&mut self) {
        #[cfg(debug_assertions)]
        if !self.debug_key
            && self
                .device_state
                .get_keys()
                .contains(&AppKeycode::F7.into())
        {
            self.debug_key = true;
            self.rhai_interface.test_script();
        } else if self.debug_key
            && !self
                .device_state
                .get_keys()
                .contains(&AppKeycode::F7.into())
        {
            self.debug_key = false;
        }
    }

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
