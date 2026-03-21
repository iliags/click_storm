use cs_hal::input::keycode::AppKeycode;
use egui::Id;
use egui_snarl::{
    Snarl,
    ui::{SnarlStyle, SnarlWidget},
};

use crate::{
    graph::{ClickStormNode, ClickStormViewer},
    localization::locale_text::LocaleText,
    settings::user_settings::UserSettings,
    ui::UIPanel,
};

pub const NODE_GRAPH_PANEL_KEY: &str = "nodegraph_panel";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct GraphPanel {
    #[serde(skip)]
    hotkey_code: AppKeycode,

    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    style: SnarlStyle,

    #[serde(skip)]
    snarl: Snarl<ClickStormNode>,
}

impl Default for GraphPanel {
    fn default() -> Self {
        Self {
            hotkey_code: AppKeycode::F6,
            language: LocaleText::default(),
            style: SnarlStyle::default(),
            snarl: Snarl::new(),
        }
    }
}

impl UIPanel for GraphPanel {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("Node graph goes here");
        egui::CentralPanel::default().show(ctx, |ui| {
            SnarlWidget::new()
                .id(Id::new("snarl-demo"))
                .style(self.style)
                .show(&mut self.snarl, &mut ClickStormViewer, ui);
        });
    }

    fn show_settings(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui) {}

    fn start(&mut self) {
        eprintln!("Start not implemented");
    }

    fn stop(&mut self) {
        eprintln!("Stop not implemented");
    }

    fn toggle(&mut self) {
        eprintln!("Toggle not implemented");
    }

    fn is_running(&self) -> bool {
        eprintln!("IsRunning not implemented");
        false
    }

    fn handle_input(&mut self) {
        // Hotkeys should go here
    }

    fn reset(&mut self) {
        eprintln!("Reset not implemented");
    }

    fn exit(&mut self) {
        eprintln!("Exit not implemented");
    }

    fn set_language(&mut self, language: LocaleText) {
        self.language = language.clone();
    }

    fn set_user_settings(&mut self, _user_settings: UserSettings) {
        eprintln!("User settings for node graph unused.")
    }

    fn name(&self) -> String {
        self.get_locale_string("graph").to_owned()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_hotkey(&mut self, hotkey: AppKeycode) {
        self.hotkey_code = hotkey;
    }
}

impl GraphPanel {
    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }

    // TODO: Move to trait
    pub fn load(&mut self, _value: GraphPanel) {}
}
