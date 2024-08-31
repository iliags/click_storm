use cs_hal::input::keycode::AppKeycode;

use crate::localization::locale_text::LocaleText;

mod output_log;

/// Clicker panel
pub mod clicker;

/// Script panel
#[cfg(feature = "scripting")]
pub mod script;

pub(crate) trait UIPanel {
    /// Show the panel
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);

    /// Show settings
    fn show_settings(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);

    /// Start the relevant action
    fn start(&mut self);

    /// Stop the relevant action
    fn stop(&mut self);

    /// Toggle the relevant action
    fn toggle(&mut self);

    /// Get if the panel is running an action
    fn is_running(&self) -> bool;

    /// Handle input
    fn handle_input(&mut self);

    /// Reset the panel
    fn reset(&mut self);

    /// When exiting the application
    fn exit(&mut self);

    /// Set the language
    fn set_language(&mut self, language: LocaleText);

    /// Get the panel name
    #[allow(dead_code)]
    fn name(&self) -> String;

    fn as_any(&mut self) -> &mut dyn std::any::Any;

    fn set_hotkey(&mut self, hotkey: AppKeycode);

    fn can_start(&self) -> bool {
        true
    }
}
