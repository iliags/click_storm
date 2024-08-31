use crate::localization::locale_text::LocaleText;
use cs_hal::input::keycode::AppKeycode;

mod output_log;

mod script_editor;

/// Clicker panel
pub mod clicker;

/// Script panel
#[cfg(feature = "scripting")]
pub mod script;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Pane {
    OutputLog(output_log::OutputLogPane),
    ScriptEditor(script_editor::ScriptEditorPane),
}

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TreeBehavior {}

/*
impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::OutputLog(output_log) => {
                output_log.ui(ui);
            }
            Pane::ScriptEditor(script_editor) => {
                script_editor.ui(ui);
            }
        }

        egui_tiles::UiResponse::None

        /*
        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
         */
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        // TODO: Localized text versions
        match pane {
            Pane::OutputLog(_) => "Output Log".into(),
            Pane::ScriptEditor(_) => "Script Editor".into(),
        }
    }
}
 */
