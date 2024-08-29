#![allow(dead_code, unused_imports)]

use cs_scripting::rhai_interface::RhaiInterface;
use device_query::DeviceQuery;
use egui::Margin;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use super::UIPanel;
use crate::localization::locale_text::LocaleText;

pub const SCRIPT_PANEL_KEY: &str = "script_panel";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    is_running: Arc<AtomicBool>,

    #[serde(skip)]
    script: String,

    #[serde(skip)]
    thread: Option<JoinHandle<()>>,

    #[serde(skip)]
    finished: Arc<AtomicBool>,

    #[serde(skip)]
    device_state: device_query::DeviceState,

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
            language: LocaleText::default(),
            is_running: Arc::new(AtomicBool::new(false)),
            script: String::new(),
            thread: None,
            finished: Arc::new(AtomicBool::new(false)),
            device_state: device_query::DeviceState::new(),

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

        // Test buttons for development
        #[cfg(debug_assertions)]
        {
            /*
            ui.horizontal(|ui| {
                if ui.button("Test Print").clicked() {
                    self.rhai_interface.test_hello();
                }

                if ui.button("Load test script").clicked() {
                    self.script = cs_scripting::rhai_interface::TEST_SCRIPT.to_string();
                }
            });
             */
        }

        ui.group(|ui| {
            ui.columns(3, |cols| {
                cols[0].group(|ui| {
                    //ui.heading("Script");
                    ui.label(egui::RichText::new("MyScript.rhai"));

                    ui.horizontal(|ui| {
                        if ui.button("Open").clicked() {
                            // Open file dialog
                        }
                        ui.separator();

                        if ui.button("Reload").clicked() {
                            // Reload script
                        }
                    });
                });
                cols[2].group(|ui| {
                    //ui.heading("Misc");
                    let cursor_pos = self.device_state.get_mouse().coords;
                    let cursor_pos = format!("🖱: ({}, {})", cursor_pos.0, cursor_pos.1);
                    ui.label(egui::RichText::new(cursor_pos).size(18.0));
                    ui.label(egui::RichText::new("ASDF").size(18.0));
                });
            });
        });

        ui.group(|ui| {
            ui.label("Script output");
        });
    }

    fn start(&mut self) {
        if self.is_running() {
            return;
        }

        println!("Starting script");
        self.is_running.store(true, Ordering::SeqCst);

        let finished = self.finished.clone();

        let script = self.script.clone();

        self.thread = Some(thread::spawn(move || {
            let mut rhai_interface = RhaiInterface::new();
            rhai_interface.initialize();

            match rhai_interface.run_script(&script) {
                Ok(_) => {
                    println!("Script finished");
                    finished.store(true, Ordering::SeqCst);
                }
                Err(err) => {
                    // TODO: Push error to user facing console
                    eprintln!("Error: {}", err);
                    finished.store(true, Ordering::SeqCst);
                }
            }
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
                self.rhai_interface.test_script();
            } else if self.debug_key && !hotkey_pressed {
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

    fn name(&self) -> &str {
        // TODO: Get the localized string
        "Script"
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn can_start(&self) -> bool {
        !self.script.is_empty()
    }
}

impl ScriptPanel {
    pub fn load(&mut self, _value: ScriptPanel) {
        // TODO
    }
}
