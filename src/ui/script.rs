#![allow(dead_code, unused_imports)]

use cs_hal::input::keycode::AppKeycode;
use cs_scripting::{output_log::OutputLog, rhai_interface::RhaiInterface, script::Script};
use device_query::DeviceQuery;
use egui::{Margin, TextBuffer};

use egui_code_editor::{CodeEditor, ColorTheme, Syntax, DEFAULT_THEMES};
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

// Note: It seems like the tiles are intended to be separate structs. The main struct needs to generate the tiles then pass the relevant data to the tiles.

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ScriptPanel {
    font_size: f32,
    theme: usize,

    script: Script,

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

    #[serde(skip)]
    output_log: Arc<Mutex<OutputLog>>,

    #[serde(skip)]
    panes: egui_tiles::Tree<Pane>,

    // TODO: Debug only
    #[serde(skip)]
    rhai_interface: RhaiInterface,

    // TODO: Debug only
    #[serde(skip)]
    debug_key: bool,
}

impl Default for ScriptPanel {
    fn default() -> Self {
        // Example gen
        let mut next_view_nr = 0;
        let mut gen_pane = || {
            let pane = Pane { nr: next_view_nr };
            next_view_nr += 1;
            pane
        };

        let mut tiles = egui_tiles::Tiles::default();

        let _log_panel = LogPanel {
            output_log: Arc::new(Mutex::new(OutputLog::new())),
        };

        let mut tabs = vec![];
        tabs.push({
            //let children = (0..2).map(|_| tiles.insert_pane(gen_pane())).collect();
            let children = vec![tiles.insert_pane(gen_pane()), tiles.insert_pane(gen_pane())];
            tiles.insert_vertical_tile(children)
        });

        let root = tiles.insert_tab_tile(tabs);

        let tree = egui_tiles::Tree::new("my_tree", root, tiles);
        // Example gen end

        Self {
            font_size: 13.0,
            theme: 7,
            hotkey_code: AppKeycode::F6,
            save_gate: DoOnceGate::default(),
            language: LocaleText::default(),
            is_running: Arc::new(AtomicBool::new(false)),
            script: Script::default(),
            thread: None,
            finished: Arc::new(AtomicBool::new(false)),
            device_state: device_query::DeviceState::new(),
            output_log: Arc::new(Mutex::new(OutputLog::new())),
            panes: tree,

            // TODO: Debug only
            rhai_interface: RhaiInterface::new(),

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

        let mut behavior = TreeBehavior {};

        self.panes.ui(&mut behavior, ui);

        /*

        ui.group(|ui| {
            ui.columns(3, |cols| {
                cols[0].group(|ui| {
                    ui.vertical_centered(|ui| {
                        // Use the filename as the title
                        let file_name = match self.script.get_filename() {
                            Some(file_name) => file_name,
                            None => self.get_locale_string("none"),
                        };

                        let file_name = if self.script.has_changes() {
                            format!("{}*", file_name)
                        } else {
                            file_name
                        };

                        ui.label(file_name);
                    });

                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(!self.script.is_default(), |ui| {
                            if ui.button(self.get_locale_string("new")).clicked() {
                                // TODO: Check if there are changes, show a dialog if so

                                self.script = Script::default();
                                self.script.set_script(NEW_SCRIPT.to_string());
                            }
                        });

                        ui.separator();

                        if ui.button(self.get_locale_string("open")).clicked() {
                            // TODO: Check for changes
                            self.load_file();
                        }

                        ui.separator();

                        ui.add_enabled_ui(self.script.has_path(), |ui| {
                            if ui
                                .button(self.get_locale_string("reload"))
                                .on_hover_text(self.get_locale_string("reload_tooltip"))
                                .clicked()
                            {
                                self.script.reload_from_disk();
                            }
                        });

                        ui.separator();

                        if ui.button(self.get_locale_string("save")).clicked() {
                            self.save_file();
                        }
                    });
                });
                cols[1].group(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(self.get_locale_string("actions"));
                    });

                    ui.columns(2, |cols| {
                        // Note: Not localized text
                        let keycode: device_query::Keycode = self.hotkey_code.into();
                        let key_code_text = format!(" ({})", keycode).to_owned();
                        cols[0].vertical_centered(|ui| {
                            let enabled = !self.is_running() && self.can_start();

                            let mut start_text = self.get_locale_string("run");
                            start_text.push_str(&key_code_text);

                            let start_button =
                                ui.add_enabled(enabled, egui::Button::new(start_text));

                            if start_button.clicked() {
                                self.start();
                            }
                        });
                        cols[1].vertical_centered(|ui| {
                            let mut stop_text = self.get_locale_string("stop");
                            stop_text.push_str(&key_code_text);

                            if ui.button(stop_text).clicked() {
                                self.stop();
                            }
                            ui.end_row();
                        });
                    });
                });
                cols[2].group(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(self.get_locale_string("misc"));
                    });
                    let cursor_pos = self.device_state.get_mouse().coords;
                    let cursor_pos = format!("🖱: ({}, {})", cursor_pos.0, cursor_pos.1);
                    ui.label(egui::RichText::new(cursor_pos).size(16.0));
                });
            });
        });

        egui::ScrollArea::vertical()
            .id_source("scripting")
            .show(ui, |ui| {
                /*
                egui::CollapsingHeader::new(self.get_locale_string("script"))
                    .default_open(true)
                    .show(ui, |ui| {
                     */
                ui.group(|ui| {
                    egui::ScrollArea::vertical()
                        .id_source("text_editor")
                        .max_height(256.0)
                        .show(ui, |ui| {
                            CodeEditor::default()
                                .id_source("code editor")
                                .with_rows(12)
                                .with_fontsize(self.font_size)
                                .with_theme(DEFAULT_THEMES[self.theme])
                                .with_syntax(Syntax::rust())
                                .with_numlines(true)
                                .show(ui, self.script.get_mut());
                        });
                });
                //});

                /*
                egui::TopBottomPanel::bottom("output_log_panel")
                    .min_height(105.0)
                    .show(ctx, |ui| {
                     */
                /*
                egui::CollapsingHeader::new(self.get_locale_string("log"))
                    .default_open(true)
                    .show(ui, |ui| {
                     */
                ui.group(|ui| {
                    egui::ScrollArea::vertical()
                        .id_source("output_log")
                        .show(ui, |ui| {
                            let output_log = self.output_log.lock().unwrap();
                            let mut text_buffer = output_log.get_log_copy();

                            ui.add(
                                egui::TextEdit::multiline(&mut text_buffer)
                                    .font(egui::TextStyle::Monospace)
                                    .code_editor()
                                    .desired_rows(6)
                                    .lock_focus(true)
                                    .desired_width(f32::INFINITY)
                                    .cursor_at_end(true),
                            );
                        });
                });
                //});
                //});
            });
             */
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
        let current_keys = self.device_state.get_keys();

        let save_shortcut = current_keys.contains(&AppKeycode::LControl.into())
            && current_keys.contains(&AppKeycode::S.into());

        if save_shortcut && self.save_gate.is_inactive() {
            self.save_gate.set_waiting();
            println!("Saving script");
            self.save_file();
        } else if !save_shortcut && self.save_gate.is_waiting_for_reset() {
            self.save_gate.reset();
        }

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

    fn show_settings(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.separator();
        ui.menu_button(self.get_locale_string("script"), |ui| {
            ui.horizontal(|ui| {
                let font_size_text = self.get_locale_string("font_size");
                ui.label(font_size_text);

                ui.separator();

                if ui.button(RESET_EMOJI).clicked() {
                    self.font_size = 13.0;
                }
            });
            ui.add(egui::Slider::new(&mut self.font_size, 8.0..=24.0));

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(self.get_locale_string("theme"));

                ui.separator();

                if ui.button(RESET_EMOJI).clicked() {
                    self.theme = 7;
                }
            });

            ui.horizontal(|ui| {
                egui::Grid::new("themes").num_columns(2).show(ui, |ui| {
                    for (i, theme) in DEFAULT_THEMES.iter().enumerate() {
                        ui.radio_value(&mut self.theme, i, theme.name);

                        if i % 3 == 1 {
                            ui.end_row();
                        }
                    }
                });
            });
        });
    }
}

impl ScriptPanel {
    pub fn load(&mut self, value: ScriptPanel) {
        self.font_size = value.font_size;
        self.theme = value.theme;
        self.script = value.script;
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

            match files {
                Some(file) => {
                    self.script.set_script_path(Some(file));
                    self.script.save();
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

        self.script.load(files);
    }
}

#[derive(Debug)]
struct Pane {
    nr: usize,
}

struct TreeBehavior {}

struct LogPanel {
    output_log: Arc<Mutex<OutputLog>>,
}

impl egui_tiles::Behavior<Pane> for LogPanel {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        let color = egui::epaint::Hsva::new(0.103 * pane.nr as f32, 0.3, 0.1, 1.0);
        ui.painter().rect_filled(ui.max_rect(), 0.0, color);

        ui.label(format!("The contents of pane {}.", pane.nr));

        egui::ScrollArea::vertical()
            .id_source("output_log")
            .show(ui, |ui| {
                let output_log = self.output_log.lock().unwrap();
                let mut text_buffer = output_log.get_log_copy();

                ui.add(
                    egui::TextEdit::multiline(&mut text_buffer)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(6)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .cursor_at_end(true),
                );
            });

        // You can make your pane draggable like so:
        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        format!("Pane {}", pane.nr).into()
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        // Give each pane a unique color:
        let color = egui::epaint::Hsva::new(0.103 * pane.nr as f32, 0.3, 0.1, 1.0);
        ui.painter().rect_filled(ui.max_rect(), 0.0, color);

        ui.label(format!("The contents of pane {}.", pane.nr));

        // You can make your pane draggable like so:
        if ui
            .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        format!("Pane {}", pane.nr).into()
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut next_view_nr = 0;
    let mut gen_pane = || {
        let pane = Pane { nr: next_view_nr };
        next_view_nr += 1;
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push({
        let children = (0..2).map(|_| tiles.insert_pane(gen_pane())).collect();
        tiles.insert_vertical_tile(children)
    });

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("my_tree", root, tiles)
}
