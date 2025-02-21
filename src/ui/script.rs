#![allow(dead_code, unused_imports)]

use cs_hal::input::keycode::AppKeycode;
use cs_scripting::{
    output_log::OutputLog,
    rhai_interface::RhaiInterface,
    script::{self, Script},
};
use device_query::DeviceQuery;
use egui::{output, Margin, TextBuffer};

use egui_code_editor::{CodeEditor, ColorTheme, Syntax, DEFAULT_THEMES};
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use rfd::FileDialog;

use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, current, JoinHandle},
};

use super::UIPanel;
use crate::{do_once::DoOnceGate, localization::locale_text::LocaleText};

pub const SCRIPT_PANEL_KEY: &str = "script_panel";
const NEW_SCRIPT: &str = "let cs = new_click_storm();\n\n";
const RESET_EMOJI: &str = "⟳";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    panels: Panels,
    tree: DockState<String>,

    #[serde(skip)]
    hotkey_code: AppKeycode,

    #[serde(skip)]
    save_gate: DoOnceGate,

    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    is_running: Arc<AtomicBool>,

    #[serde(skip)]
    thread: Option<JoinHandle<()>>,

    #[serde(skip)]
    finished: Arc<AtomicBool>,

    #[serde(skip)]
    device_state: device_query::DeviceState,

    // Note: Maybe change to RwLock since it is only written to in the worker thread
    #[serde(skip)]
    output_log: Arc<Mutex<OutputLog>>,
}

impl Default for ScriptPanel {
    fn default() -> Self {
        let mut new_self = Self {
            panels: Panels::default(),
            tree: DockState::new(vec![]),

            // Serde skips
            hotkey_code: AppKeycode::F6,
            save_gate: DoOnceGate::default(),
            language: LocaleText::default(),
            is_running: Arc::new(AtomicBool::new(false)),
            thread: None,
            finished: Arc::new(AtomicBool::new(false)),
            device_state: device_query::DeviceState::new(),
            output_log: Arc::new(Mutex::new(OutputLog::new())),
        };

        let mut dock_state = DockState::new(vec!["ScriptEditor".to_owned()]);

        let [_, b] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.65,
            vec!["OutputLog".to_owned()],
        );

        let [_, _] = dock_state
            .main_surface_mut()
            .split_right(b, 0.5, vec!["Misc".to_owned()]);

        new_self.tree = dock_state;

        new_self
    }
}

impl UIPanel for ScriptPanel {
    fn show(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        if self.finished.load(Ordering::SeqCst) {
            self.finished.store(false, Ordering::SeqCst);
            self.stop();
        }

        egui::TopBottomPanel::top("top_panel_script").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(self.get_locale_string("new")).clicked() {
                        // TODO: Check if there are changes, show a dialog if so

                        self.panels.script = Script::default();
                        self.panels.script.set_script(NEW_SCRIPT);
                    }

                    if ui.button(self.get_locale_string("open")).clicked() {
                        // TODO: Check for changes
                        self.load_file();
                    }

                    ui.add_enabled_ui(self.panels.script.has_path(), |ui| {
                        if ui
                            .button(self.get_locale_string("reload"))
                            .on_hover_text(self.get_locale_string("reload_tooltip"))
                            .clicked()
                        {
                            self.panels.script.reload_from_disk();
                        }
                    });

                    if ui.button(self.get_locale_string("save")).clicked() {
                        self.save_file();
                    }

                    ui.separator();

                    let keycode: device_query::Keycode = self.hotkey_code.into();
                    let key_code_text = format!(" ({})", keycode).to_owned();

                    ui.add_enabled_ui(!self.is_running(), |ui| {
                        let start_text = format!("▶{}", key_code_text);
                        if ui
                            .button(start_text)
                            .on_hover_text_at_pointer(self.get_locale_string("run"))
                            .clicked()
                        {
                            self.start();
                        }
                    });

                    ui.add_enabled_ui(self.is_running(), |ui| {
                        let stop_text = format!("⏹{}", key_code_text);
                        if ui
                            .button(stop_text)
                            .on_hover_text_at_pointer(self.get_locale_string("stop"))
                            .clicked()
                        {
                            self.stop();
                        }
                    });
                });
            });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.panels);
    }

    fn start(&mut self) {
        if self.is_running() {
            return;
        }

        println!("Starting script");
        self.is_running.store(true, Ordering::SeqCst);

        let finished = self.finished.clone();
        let script = self.panels.script.get_copy();

        // Clear the output log
        self.output_log.lock().unwrap().clear();

        // Clone the output log for the thread
        let output_log = self.output_log.clone();

        self.thread = Some(thread::spawn(move || {
            let mut rhai_interface = RhaiInterface::new();

            let mut result_message = String::new();
            match rhai_interface.run_script(&script, output_log.clone()) {
                Ok(_) => {
                    result_message.push_str("Script finished successfully");
                }
                Err(err) => {
                    result_message.push_str(&err);
                }
            }

            let mut output_log = output_log.lock().unwrap();
            output_log.log(&result_message);

            println!("{}", result_message);

            finished.store(true, Ordering::SeqCst);
        }));
    }

    fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    fn toggle(&mut self) {
        println!("Toggling script");

        if self.is_running() {
            self.stop();
        } else {
            self.start();
        }
    }

    fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn handle_input(&mut self) {
        let current_keys = self.device_state.get_keys();

        let save_shortcut = current_keys.contains(&AppKeycode::LControl.into())
            && current_keys.contains(&AppKeycode::S.into());

        if save_shortcut && !self.save_gate.is_waiting_for_reset() {
            self.save_gate.set_waiting();
            println!("Saving script");
            self.save_file();
        } else if !save_shortcut && self.save_gate.is_waiting_for_reset() {
            self.save_gate.reset();
        }
    }

    fn reset(&mut self) {
        let mut dock_state = DockState::new(vec!["ScriptEditor".to_owned()]);

        let [_, b] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.65,
            vec!["OutputLog".to_owned()],
        );

        let [_, _] = dock_state
            .main_surface_mut()
            .split_right(b, 0.5, vec!["Misc".to_owned()]);

        self.tree = dock_state;
    }

    fn exit(&mut self) {
        println!("Script shutting down");
        self.stop();
    }

    fn set_language(&mut self, language: LocaleText) {
        self.language = language.clone();
        self.panels.language = language;
    }

    fn name(&self) -> String {
        self.get_locale_string("script").to_owned()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn can_start(&self) -> bool {
        !self.panels.script.is_empty()
    }

    fn set_hotkey(&mut self, hotkey: AppKeycode) {
        self.hotkey_code = hotkey;
    }

    fn show_settings(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.separator();
        ui.menu_button(self.get_locale_string("script"), |ui| {
            ui.horizontal(|ui| {
                let font_size_text = self.get_locale_string("font_size");
                ui.label(font_size_text);

                ui.separator();

                if ui.button(RESET_EMOJI).clicked() {
                    self.panels.font_size = 13.0;
                }
            });
            ui.add(egui::Slider::new(&mut self.panels.font_size, 8.0..=24.0));

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(self.get_locale_string("theme"));

                ui.separator();

                if ui.button(RESET_EMOJI).clicked() {
                    self.panels.theme = 7;
                }
            });

            ui.horizontal(|ui| {
                egui::Grid::new("themes").num_columns(2).show(ui, |ui| {
                    for (i, theme) in DEFAULT_THEMES.iter().enumerate() {
                        ui.radio_value(&mut self.panels.theme, i, theme.name);

                        if i % 3 == 1 {
                            ui.end_row();
                        }
                    }
                });
            });
        });
    }

    fn set_user_settings(&mut self, _user_settings: crate::settings::user_settings::UserSettings) {
        // Not used for now
    }
}

impl ScriptPanel {
    pub fn load(&mut self, value: ScriptPanel) {
        self.tree = value.tree;
        self.panels = value.panels;
        self.panels.output_log = self.output_log.clone();
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }

    fn save_file(&mut self) {
        if self.panels.script.save() {
            let files = FileDialog::new()
                .add_filter("rhai", &["rhai"])
                .set_directory("/")
                .save_file();

            println!("{:?}", files);

            match files {
                Some(file) => {
                    self.panels.script.set_script_path(Some(file));
                    self.panels.script.save();
                }
                None => {
                    println!("No file selected");
                }
            }
        }
    }

    fn load_file(&mut self) {
        let files = FileDialog::new()
            .add_filter("rhai", &["rhai"])
            .set_directory("/")
            .pick_file();

        println!("{:?}", files);

        self.panels.script.load(files);
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Panels {
    script: Script,
    font_size: f32,
    theme: usize,

    #[serde(skip)]
    output_log: Arc<Mutex<OutputLog>>,

    #[serde(skip)]
    device_state: device_query::DeviceState,

    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    hotkey_code: AppKeycode,
}

impl Default for Panels {
    fn default() -> Self {
        let mut script = Script::default();
        script.set_script(NEW_SCRIPT);

        Self {
            script,
            font_size: 13.0,
            theme: 7,
            output_log: Arc::new(Mutex::new(OutputLog::new())),
            device_state: device_query::DeviceState::new(),
            language: LocaleText::default(),
            hotkey_code: AppKeycode::F6,
        }
    }
}

impl TabViewer for Panels {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab.as_str() {
            "OutputLog" => self.get_locale_string("log").into(),
            "ScriptEditor" => {
                /* TODO: This causes the code editor to lose focus
                let file_name = match self.script.get_filename() {
                    Some(file_name) => file_name,
                    None => self.get_locale_string("none"),
                };

                let file_name = if self.script.has_changes() {
                    format!("{}*", file_name)
                } else {
                    file_name
                };

                //self.get_locale_string("script").into()
                let tab_name = format!("{} - {}", self.get_locale_string("script"), file_name);

                tab_name.into()
                 */

                self.get_locale_string("script").into()
            }
            "Misc" => self.get_locale_string("misc").into(),
            _ => "???".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "OutputLog" => {
                self.output_log(ui);
            }
            "ScriptEditor" => {
                self.script_editor(ui);
            }
            "Misc" => {
                self.settings(ui);
            }
            _ => {}
        }
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
}

impl Panels {
    pub fn new(output_log: Arc<Mutex<OutputLog>>) -> Self {
        Self {
            output_log,
            ..Default::default()
        }
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }

    fn output_log(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_salt("output_log")
            .show(ui, |ui| {
                let output_log = self.output_log.lock().unwrap();
                let mut text_buffer = output_log.get_log_copy();

                ui.add(
                    egui::TextEdit::multiline(&mut text_buffer)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(8)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .cursor_at_end(true),
                );
            });
    }
    fn script_editor(&mut self, ui: &mut egui::Ui) {
        CodeEditor::default()
            .id_source("code_editor")
            .with_rows(20)
            .with_fontsize(self.font_size)
            .with_theme(DEFAULT_THEMES[self.theme])
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .show(ui, self.script.get_mut());
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        let cursor_pos = self.device_state.get_mouse().coords;
        let cursor_pos = format!("🖱: ({}, {})", cursor_pos.0, cursor_pos.1);
        ui.label(egui::RichText::new(cursor_pos).size(16.0));
    }
}
