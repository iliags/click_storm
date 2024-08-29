use crate::localization::locale_text::LocaleText;

/// Clicker panel
pub mod clicker;

/// Script panel
pub mod script;

#[derive(Debug, Default, PartialEq)]
pub enum UIMode {
    #[default]
    Clicker,
    Script,
}

pub(crate) trait UIPanel {
    /// Show the panel
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);

    /// Start the relevant action
    fn start(&mut self);

    /// Stop the relevant action
    fn stop(&mut self);

    /// Handle input
    fn handle_input(&mut self);

    /// Reset the panel
    fn reset(&mut self);

    /// When exiting the application
    fn exit(&mut self);

    /// Set the language
    fn set_language(&mut self, language: LocaleText);
}
