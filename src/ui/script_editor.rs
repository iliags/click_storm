use cs_scripting::script::Script;
use egui_code_editor::{CodeEditor, Syntax, DEFAULT_THEMES};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ScriptEditorPane {
    script: Script,
    font_size: f32,
    theme: usize,
}

impl Default for ScriptEditorPane {
    fn default() -> Self {
        Self {
            script: Script::default(),
            font_size: 13.0,
            theme: 7,
        }
    }
}

impl ScriptEditorPane {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        /*
        egui::ScrollArea::vertical()
            .id_source("text_editor")
            .max_height(256.0)
            .show(ui, |ui| {
             */
        CodeEditor::default()
            .id_source("code editor")
            .with_rows(18)
            .with_fontsize(self.font_size)
            .with_theme(DEFAULT_THEMES[self.theme])
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .show(ui, self.script.get_mut());
        //});
    }
}
