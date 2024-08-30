#![allow(dead_code, unused_imports)]

use cs_hal::input::keycode::AppKeycode;
use cs_scripting::{output_log::OutputLog, rhai_interface::RhaiInterface, script::Script};
use device_query::DeviceQuery;
use egui::{Margin, TextBuffer};

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use rfd::FileDialog;

use std::{
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::UIPanel;
use crate::localization::locale_text::LocaleText;

pub const SCRIPT_PANEL_KEY: &str = "script_panel";

// TODO: Code editor settings
// - Color theme
// - Font size

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    #[serde(skip)]
    hotkey_code: AppKeycode,

    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    is_running: Arc<AtomicBool>,

    #[serde(skip)]
    script: Script,

    #[serde(skip)]
    thread: Option<JoinHandle<()>>,

    #[serde(skip)]
    finished: Arc<AtomicBool>,

    #[serde(skip)]
    device_state: device_query::DeviceState,

    #[serde(skip)]
    output_log: Arc<Mutex<OutputLog>>,

    // TODO: Debug only
    #[serde(skip)]
    rhai_interface: RhaiInterface,

    // TODO: Debug only
    #[serde(skip)]
    debug_key: bool,
}

impl Default for ScriptPanel {
    fn default() -> Self {
        let mut rhai_interface = RhaiInterface::new();
        rhai_interface.initialize();

        Self {
            hotkey_code: AppKeycode::F6.into(),
            language: LocaleText::default(),
            is_running: Arc::new(AtomicBool::new(false)),
            script: Script::default(),
            thread: None,
            finished: Arc::new(AtomicBool::new(false)),
            device_state: device_query::DeviceState::new(),
            output_log: Arc::new(Mutex::new(OutputLog::new())),

            // TODO: Debug only
            rhai_interface,

            debug_key: false,
        }
    }
}

impl UIPanel for ScriptPanel {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.finished.load(Ordering::SeqCst) {
            self.finished.store(false, Ordering::SeqCst);
            self.stop();
        }

        ui.group(|ui| {
            ui.columns(3, |cols| {
                cols[0].group(|ui| {
                    ui.label(egui::RichText::new("MyScript.rhai"));
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(!self.script.is_default(), |ui| {
                            if ui.button(self.get_locale_string("new")).clicked() {
                                // TODO: Check if there are changes, show a dialog if so

                                self.script = Script::default();
                            }
                        });

                        ui.separator();

                        if ui.button(self.get_locale_string("open")).clicked() {
                            // TODO: Check for changes
                            self.load_file();
                        }

                        ui.separator();

                        if ui.button(self.get_locale_string("save")).clicked() {
                            self.save_file();
                        }
                    });
                });
                cols[1].group(|ui| {
                    ui.heading("Actions");
                });
                cols[2].group(|ui| {
                    //ui.heading("Misc");
                    let cursor_pos = self.device_state.get_mouse().coords;
                    let cursor_pos = format!("🖱: ({}, {})", cursor_pos.0, cursor_pos.1);
                    ui.label(egui::RichText::new(cursor_pos).size(16.0));
                    //ui.label(egui::RichText::new("ASDF").size(18.0));
                });
            });
        });

        egui::ScrollArea::vertical()
            .id_source("scripting")
            .show(ui, |ui| {
                egui::CollapsingHeader::new(self.get_locale_string("script"))
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.group(|ui| {
                            egui::ScrollArea::vertical()
                                .id_source("text_editor")
                                .max_height(256.0)
                                .show(ui, |ui| {
                                    //let mut text_buffer = self.script.clone().to_string();
                                    CodeEditor::default()
                                        .id_source("code editor")
                                        .with_rows(12)
                                        .with_fontsize(13.0)
                                        .with_theme(ColorTheme::SONOKAI)
                                        .with_syntax(Syntax::rust())
                                        .with_numlines(true)
                                        .show(ui, self.script.get_mut());
                                });
                        });
                    });

                egui::CollapsingHeader::new(self.get_locale_string("log"))
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.group(|ui| {
                            egui::ScrollArea::vertical()
                                .id_source("output_log")
                                .show(ui, |ui| {
                                    let output_log = self.output_log.lock().unwrap();
                                    let mut text_buffer = output_log.get_log_copy();

                                    ui.add(
                                        egui::TextEdit::multiline(&mut text_buffer)
                                            .font(egui::TextStyle::Monospace) // for cursor height
                                            .code_editor()
                                            .desired_rows(6)
                                            .lock_focus(true)
                                            .desired_width(f32::INFINITY)
                                            .cursor_at_end(true),
                                    );
                                });
                        });
                    });
            });
    }

    fn start(&mut self) {
        if self.is_running() {
            return;
        }

        println!("Starting script");
        self.is_running.store(true, Ordering::SeqCst);

        let finished = self.finished.clone();
        let script = self.script.get_copy();

        // Clear the output log
        self.output_log.lock().unwrap().clear();

        // Clone the output log for the thread
        let output_log = self.output_log.clone();

        self.thread = Some(thread::spawn(move || {
            let mut rhai_interface = RhaiInterface::new();
            rhai_interface.initialize();

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
        // TODO: Save keybinding

        #[cfg(debug_assertions)]
        {
            use cs_hal::input::keycode::AppKeycode;
            use device_query::DeviceQuery;

            let hotkey_pressed = self
                .device_state
                .get_keys()
                .contains(&AppKeycode::F7.into());

            if !self.debug_key && hotkey_pressed {
                self.debug_key = true;
                //self.rhai_interface.test_script();
                self.script
                    .set_script(cs_scripting::rhai_interface::TEST_SCRIPT.to_string());
                self.start();
            } else if self.debug_key && !hotkey_pressed {
                self.debug_key = false;
            }

            let hotkey_pressed2 = self
                .device_state
                .get_keys()
                .contains(&AppKeycode::F8.into());

            if !self.debug_key && hotkey_pressed2 {
                self.debug_key = true;

                self.script
                    .set_script(cs_scripting::rhai_interface::TEST_SCRIPT.to_string());
            } else if self.debug_key && !hotkey_pressed2 {
                self.debug_key = false;
            }
        }
    }

    fn reset(&mut self) {}

    fn exit(&mut self) {
        println!("Script shutting down");
        self.stop();
    }

    fn set_language(&mut self, language: LocaleText) {
        self.language = language;
    }

    fn name(&self) -> String {
        self.get_locale_string("script").to_owned()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn can_start(&self) -> bool {
        !self.script.is_empty()
    }

    fn set_hotkey(&mut self, hotkey: AppKeycode) {
        self.hotkey_code = hotkey;
    }
}

impl ScriptPanel {
    pub fn load(&mut self, _value: ScriptPanel) {
        // TODO
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }

    fn save_file(&mut self) {
        if self.script.save() {
            let files = FileDialog::new()
                .add_filter("rhai", &["rhai"])
                .set_directory("/")
                .save_file();

            println!("{:?}", files);

            self.script.set_script_path(files);
        }
    }

    fn load_file(&mut self) {
        let files = FileDialog::new()
            .add_filter("rhai", &["rhai"])
            .set_directory("/")
            .pick_file();

        println!("{:?}", files);

        self.script.load(files);
    }
}
