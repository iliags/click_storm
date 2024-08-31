use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use cs_hal::input::{keycode::AppKeycode, mouse_button::MouseButton, mouse_click::MouseClickType};
use device_query::{DeviceQuery, DeviceState, MouseState};
use egui::Margin;
use enigo::{Enigo, Mouse, Settings};
use strum::IntoEnumIterator;

use crate::{
    localization::locale_text::LocaleText,
    settings::{
        app_settings::AppSettings, cursor_position::CursorPosition, repeat_type::RepeatType,
    },
    worker::{self},
};

use super::UIPanel;

pub const CLICKER_PANEL_KEY: &str = "clicker_panel";

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct ClickerPanel {
    settings: AppSettings,
    cursor_position_fixed: (i32, i32),
    repeat_count: usize,

    #[serde(skip)]
    hotkey_code: AppKeycode,

    #[serde(skip)]
    language: LocaleText,

    #[serde(skip)]
    device_state: DeviceState,

    #[serde(skip)]
    display_size: (i32, i32),

    #[serde(skip)]
    picking_position: bool,

    #[serde(skip)]
    is_running: Arc<AtomicBool>,

    #[serde(skip)]
    thread: Option<JoinHandle<()>>,
}

impl Default for ClickerPanel {
    fn default() -> Self {
        // TODO: Handle error
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        let display_size = enigo
            .main_display()
            .unwrap_or_else(|_| panic!("Failed to get display size."));

        Self {
            settings: AppSettings::default(),
            language: LocaleText::default(),
            cursor_position_fixed: (0, 0),
            repeat_count: 0,
            hotkey_code: AppKeycode::F6.into(),
            device_state: DeviceState::new(),
            display_size,
            picking_position: false,
            is_running: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }
}

impl UIPanel for ClickerPanel {
    fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add_enabled_ui(!self.is_running.load(Ordering::SeqCst), |ui| {
            self.ui_interval(ui);

            ui.horizontal(|ui| {
                self.ui_click_options(ui);
                self.ui_click_repeat(ui);
            });

            self.ui_cursor_position(ui);
        });

        ui.separator();

        self.ui_actions(ui);
    }

    fn start(&mut self) {
        if self.is_running() {
            return;
        }

        self.is_running.store(true, Ordering::SeqCst);

        let settings_clone = self.settings.clone();
        let running_clone = Arc::clone(&self.is_running);

        self.thread = Some(thread::spawn(move || {
            worker::worker_thread(settings_clone, running_clone);
        }));
    }

    fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    fn handle_input(&mut self) {
        if self.picking_position {
            let mouse: MouseState = self.device_state.get_mouse();

            for press in mouse.button_pressed.iter() {
                if *press {
                    let coords = mouse.coords;
                    self.cursor_position_fixed.0 = coords.0;
                    self.cursor_position_fixed.1 = coords.1;
                    self.picking_position = false;
                    println!("Picked position: {:?}", coords);
                }
            }
        }
    }

    fn reset(&mut self) {
        self.settings.reset();
        self.cursor_position_fixed = (0, 0);
        self.repeat_count = 0;
    }

    fn exit(&mut self) {
        self.stop();
    }

    fn set_language(&mut self, language: LocaleText) {
        self.language = language;
    }

    fn toggle(&mut self) {
        if self.is_running.load(Ordering::SeqCst) {
            //println!("Stop");
            self.stop();
        } else {
            //println!("Start");
            self.start();
        }
    }

    fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn name(&self) -> String {
        self.get_locale_string("clicker").to_owned()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_hotkey(&mut self, hotkey: AppKeycode) {
        self.hotkey_code = hotkey;
    }

    fn show_settings(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui) {}
}

impl ClickerPanel {
    pub fn load(&mut self, value: ClickerPanel) {
        self.settings = value.settings;
        self.cursor_position_fixed = value.cursor_position_fixed;
        self.repeat_count = value.repeat_count;
    }

    fn ui_interval(&mut self, ui: &mut egui::Ui) {
        let interval_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.heading(self.get_locale_string("click_interval"));
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.columns(8, |cols| {
                            cols[0].centered_and_justified(|ui| {
                                ui.label(self.get_locale_string("hours"));
                            });
                            cols[1].centered_and_justified(|ui| {
                                ui.add(
                                    egui::DragValue::new(self.settings.interval_hours_mut())
                                        .range(0..=24)
                                        .speed(1),
                                );
                            });
                            cols[2].centered_and_justified(|ui| {
                                ui.label(self.get_locale_string("minutes"));
                            });
                            cols[3].centered_and_justified(|ui| {
                                ui.add(
                                    egui::DragValue::new(self.settings.interval_minutes_mut())
                                        .range(0..=60)
                                        .speed(1),
                                );
                            });
                            cols[4].centered_and_justified(|ui| {
                                ui.label(self.get_locale_string("seconds"));
                            });
                            cols[5].centered_and_justified(|ui| {
                                ui.add(
                                    egui::DragValue::new(self.settings.interval_seconds_mut())
                                        .range(0..=60)
                                        .speed(1),
                                );
                            });
                            cols[6].centered_and_justified(|ui| {
                                ui.label(self.get_locale_string("milliseconds"));
                            });
                            cols[7].centered_and_justified(|ui| {
                                ui.add(
                                    egui::DragValue::new(self.settings.interval_milliseconds_mut())
                                        .range(0..=1000)
                                        .speed(1),
                                );
                            });
                        });
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.columns(6, |cols| {
                            cols[0].centered_and_justified(|ui| {
                                ui.label(self.get_locale_string("repeat_variation"));
                            });
                            cols[1].centered_and_justified(|ui| {
                                ui.add(
                                    egui::DragValue::new(self.settings.repeat_variation_mut())
                                        .range(0..=1000)
                                        .speed(1)
                                        .clamp_to_range(true),
                                )
                                .on_hover_text_at_pointer(self.get_locale_string("variation_desc"));
                            });
                        });
                    });
                });
            });

        // Show the hover text for the interval frame
        interval_frame
            .response
            .on_hover_text(self.get_locale_string("click_interval_desc"));
    }

    fn ui_click_options(&mut self, ui: &mut egui::Ui) {
        let _ = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(self.get_locale_string("click_options"));

                    ui.centered_and_justified(|ui| {
                        // Click button name
                        let selected_button =
                            self.settings.mouse_button().as_str_locale().to_owned();

                        // Generate the combo box
                        egui::ComboBox::from_label(self.get_locale_string("mouse_button"))
                            .selected_text(self.get_locale_string(&selected_button))
                            .show_ui(ui, |ui| {
                                // Iterate over the click types
                                let mut current_value = self.settings.mouse_button().clone();
                                const MOUSE_BUTTONS: &[MouseButton] =
                                    &[MouseButton::Left, MouseButton::Right, MouseButton::Middle];
                                for mouse_button in MOUSE_BUTTONS {
                                    // Get the locale string for the click type
                                    let mouse_button_locale =
                                        self.get_locale_string(mouse_button.as_str_locale());

                                    // Select the click type
                                    ui.selectable_value(
                                        &mut current_value,
                                        mouse_button.clone(),
                                        mouse_button_locale,
                                    );
                                }
                                self.settings.mouse_button_mut().clone_from(&current_value);
                            });
                    });

                    ui.centered_and_justified(|ui| {
                        // Click type options
                        // Get the selected click type name
                        let selected_click_type =
                            self.settings.click_type().as_str_locale().to_owned();

                        // Generate the combo box
                        egui::ComboBox::from_label(self.get_locale_string("click_type"))
                            .selected_text(self.get_locale_string(&selected_click_type))
                            .show_ui(ui, |ui| {
                                // Iterate over the click types
                                let mut current_value = self.settings.click_type().clone();
                                for click_type in MouseClickType::iter() {
                                    // Get the locale string for the click type
                                    let click_type_locale =
                                        self.get_locale_string(click_type.as_str_locale());

                                    // Select the click type
                                    ui.selectable_value(
                                        &mut current_value,
                                        click_type,
                                        click_type_locale,
                                    );
                                }
                                self.settings.click_type_mut().clone_from(&current_value);
                            });
                    });
                });
            });
    }

    fn ui_click_repeat(&mut self, ui: &mut egui::Ui) {
        let _ = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(self.get_locale_string("repeat_options"));

                    ui.columns(4, |cols| {
                        cols[0].centered_and_justified(|ui| {
                            let repeat_infinite_name =
                                self.get_locale_string("repeat_until_stopped");
                            ui.radio_value(
                                self.settings.repeat_type_mut(),
                                RepeatType::RepeatUntilStopped,
                                repeat_infinite_name,
                            )
                            .on_hover_text_at_pointer(
                                self.get_locale_string("repeat_until_stopped_desc"),
                            );
                        });
                        cols[1].centered_and_justified(|ui| {
                            let repeat_turbo_text = self.get_locale_string("turbo_click");
                            ui.radio_value(
                                self.settings.repeat_type_mut(),
                                RepeatType::Turbo,
                                repeat_turbo_text,
                            )
                            .on_hover_text_at_pointer(self.get_locale_string("turbo_click_desc"));
                        });
                        cols[2].centered_and_justified(|ui| {
                            let repeat_count_name = self.get_locale_string("repeat_count");
                            let repeat_count = self.repeat_count;
                            ui.radio_value(
                                self.settings.repeat_type_mut(),
                                RepeatType::Repeat(repeat_count),
                                repeat_count_name,
                            )
                            .on_hover_text_at_pointer(self.get_locale_string("repeat_count_desc"));
                        });
                        cols[3].horizontal_centered(|ui| {
                            let mut current_count = self.repeat_count;
                            ui.add(
                                egui::DragValue::new(&mut current_count)
                                    .range(1..=1000)
                                    .speed(1)
                                    .clamp_to_range(false),
                            );

                            if current_count != self.repeat_count {
                                self.repeat_count = current_count;
                                self.settings
                                    .set_repeat_type(RepeatType::Repeat(current_count));
                            }
                        });
                    });
                });
            });
    }

    fn ui_cursor_position(&mut self, ui: &mut egui::Ui) {
        let cursor_position_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.heading(self.get_locale_string("cursor_position"));

                ui.horizontal(|ui| {
                    ui.columns(5, |cols| {
                        cols[0].centered_and_justified(|ui| {
                            // Current position radio button
                            let current_position_name = self.get_locale_string("current_position");

                            ui.radio_value(
                                self.settings.cursor_position_type_mut(),
                                CursorPosition::CurrentLocation,
                                current_position_name,
                            );
                        });
                        cols[1].centered_and_justified(|ui| {
                            // Fixed position radio button
                            let fixed_position_name = self.get_locale_string("fixed_position");
                            let current_position = self.cursor_position_fixed;
                            ui.radio_value(
                                self.settings.cursor_position_type_mut(),
                                CursorPosition::FixedLocation(
                                    current_position.0,
                                    current_position.1,
                                ),
                                fixed_position_name,
                            );
                        });
                        cols[2].centered_and_justified(|ui| {
                            let picking_text = if self.picking_position {
                                self.get_locale_string("picking_position")
                            } else {
                                self.get_locale_string("pick_position")
                            };
                            if ui
                                .button(picking_text)
                                .on_hover_text_at_pointer(
                                    self.get_locale_string("pick_position_desc"),
                                )
                                .clicked()
                            {
                                let (pos_x, pos_y) = self.cursor_position_fixed;
                                let cursor_type = CursorPosition::FixedLocation(pos_x, pos_y);
                                self.settings.set_cursor_position_type(cursor_type);

                                self.picking_position = true;
                            }
                        });
                        cols[3].centered_and_justified(|ui| {
                            let mut pos_x = self.cursor_position_fixed.0;
                            ui.add(
                                egui::DragValue::new(&mut pos_x)
                                    .range(0..=self.display_size.0)
                                    .prefix("x: ")
                                    .speed(1),
                            );
                            if pos_x != self.cursor_position_fixed.0 {
                                self.cursor_position_fixed.0 = pos_x;
                                let pos_y = self.cursor_position_fixed.1;

                                let cursor_type = CursorPosition::FixedLocation(pos_x, pos_y);
                                self.settings.set_cursor_position_type(cursor_type);
                            }
                        });
                        cols[4].centered_and_justified(|ui| {
                            let mut pos_y = self.cursor_position_fixed.1;
                            ui.add(
                                egui::DragValue::new(&mut pos_y)
                                    .range(0..=self.display_size.1)
                                    .prefix("y: ")
                                    .speed(1),
                            );
                            if pos_y != self.cursor_position_fixed.1 {
                                self.cursor_position_fixed.1 = pos_y;
                                let pos_x = self.cursor_position_fixed.0;

                                let cursor_type = CursorPosition::FixedLocation(pos_x, pos_y);
                                self.settings.set_cursor_position_type(cursor_type);
                            }
                        });
                    });
                });
            });

        cursor_position_frame
            .response
            .on_hover_text(self.get_locale_string("position_desc"));
    }

    fn ui_actions(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.columns(2, |cols| {
                // TODO: Change between run script and start click storm
                // Note: Not localized text
                let keycode: device_query::Keycode = self.hotkey_code.into();
                let key_code_text = format!(" ({})", keycode).to_owned();
                cols[0].centered_and_justified(|ui| {
                    let enabled = !self.is_running() && self.can_start();

                    let mut start_text = self.get_locale_string("start");
                    start_text.push_str(&key_code_text);

                    let start_button = ui.add_enabled(enabled, egui::Button::new(start_text));

                    if start_button.clicked() {
                        self.start();
                    }
                });
                cols[1].centered_and_justified(|ui| {
                    let mut stop_text = self.get_locale_string("stop");
                    stop_text.push_str(&key_code_text);

                    if ui.button(stop_text).clicked() {
                        self.stop();
                    }
                    ui.end_row();
                });
            });
        });
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }
}
