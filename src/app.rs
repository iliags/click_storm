use device_query::{DeviceQuery, DeviceState, MouseState};
use egui::Margin;
use enigo::{Button, Enigo, Mouse, Settings};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use strum::IntoEnumIterator;

use crate::{
    localization::language::Language,
    settings::{
        app_settings::AppSettings, cursor_position::CursorPosition, mouse_button::MouseButton,
        mouse_click::MouseClickType, repeat::RepeatType,
    },
};

const HOTKEY_CODE: device_query::Keycode = device_query::Keycode::F6;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ClickStormApp {
    settings: AppSettings,

    #[serde(skip)]
    device_state: DeviceState,

    #[serde(skip)]
    display_size: (i32, i32),

    #[serde(skip)]
    picking_position: bool,

    #[serde(skip)]
    sender: Option<Sender<ClickStormMessage>>,

    #[serde(skip)]
    is_running: Arc<AtomicBool>,

    #[serde(skip)]
    key_pressed: bool,
}

#[derive(Debug, Clone)]
enum ClickStormMessage {
    Start(AppSettings, Arc<AtomicBool>),
    Stop,
    Shutdown,
}

impl Default for ClickStormApp {
    fn default() -> Self {
        // TODO: Handle error
        // TODO: Maybe move this to an Arc<Mutex<Enigo>> for use in all threads
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        let display_size = enigo.main_display().unwrap();

        let (sender, receiver): (Sender<ClickStormMessage>, Receiver<ClickStormMessage>) =
            channel();

        thread::spawn(move || {
            // TODO: This is a total mess, clean it up
            let mut thread: Option<JoinHandle<()>> = None;
            let mut is_working = Arc::new(AtomicBool::new(false));

            loop {
                match receiver.recv() {
                    Ok(message) => {
                        match message {
                            ClickStormMessage::Start(settings, is_running) => {
                                // Start the click storm
                                //println!("Starting click storm with settings: {:?}", settings);

                                if let Some(thread) = thread.take() {
                                    is_running.store(true, Ordering::SeqCst);
                                    let _ = thread.join();
                                }

                                // Inner thread atomic
                                let doing_work = Arc::clone(&is_running);

                                // Worker thread atomic
                                is_working = Arc::clone(&is_running);
                                is_running.store(true, Ordering::SeqCst);

                                let settings_clone = Arc::clone(&Arc::new(settings));

                                // Worker thread
                                thread = Some(thread::spawn(move || {
                                    // Note: maybe move this to an arc mutex
                                    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
                                    panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
                                });

                                    // Get the time interval to sleep between clicks
                                    let sleep_duration = settings_clone.click_interval();
                                    //println!("Sleep duration: {:?}", sleep_duration);

                                    // Get the mouse button to click with
                                    let mouse_button = match settings_clone.mouse_button() {
                                        MouseButton::Left => Button::Left,
                                        MouseButton::Middle => Button::Middle,
                                        MouseButton::Right => Button::Right,
                                    };

                                    let mut current_count = 0;

                                    let move_mouse = *settings_clone.cursor_position_type()
                                        != CursorPosition::CurrentLocation;
                                    let single_click =
                                        *settings_clone.click_type() == MouseClickType::Single;

                                    let click_mouse =
                                    |enigo: &mut Enigo,
                                     mouse_button: Button,
                                     location: (i32, i32),
                                     move_mouse: bool,
                                     single_click: bool| {
                                        if move_mouse {
                                            let _ = enigo.move_mouse(
                                                location.0,
                                                location.1,
                                                enigo::Coordinate::Abs,
                                            );
                                        }

                                        // TODO: Handle error
                                        if single_click {
                                            let _ =
                                                enigo.button(mouse_button, enigo::Direction::Click);
                                        } else {
                                            let _ =enigo.button(mouse_button, enigo::Direction::Click);
                                            let _ =enigo.button(mouse_button, enigo::Direction::Click);
                                        }
                                    };

                                    while doing_work.load(Ordering::SeqCst) {
                                        //println!("Working");

                                        // Coordinates are in absolute screen coordinates
                                        let mouse_position =
                                            match settings_clone.cursor_position_type() {
                                                CursorPosition::CurrentLocation => {
                                                    // TODO: Error handling
                                                    enigo.location().unwrap_or_else(|_| {
                                                        panic!("Failed to get mouse location.")
                                                    })
                                                }
                                                CursorPosition::FixedLocation(x, y) => (*x, *y),
                                            };

                                        match settings_clone.repeat_type() {
                                            RepeatType::Repeat(count) => {
                                                if current_count > *count {
                                                    doing_work.store(false, Ordering::SeqCst);
                                                } else {
                                                    current_count += 1;

                                                    click_mouse(
                                                        &mut enigo,
                                                        mouse_button,
                                                        mouse_position,
                                                        move_mouse,
                                                        single_click,
                                                    );
                                                }
                                            }
                                            RepeatType::RepeatUntilStopped => {
                                                click_mouse(
                                                    &mut enigo,
                                                    mouse_button,
                                                    mouse_position,
                                                    move_mouse,
                                                    single_click,
                                                );
                                            }
                                        }

                                        thread::sleep(sleep_duration);
                                    }
                                }));
                            }
                            ClickStormMessage::Stop => {
                                // Stop the click storm
                                println!("Stopping click storm");
                                if let Some(thread) = thread.take() {
                                    is_working.store(false, Ordering::SeqCst);
                                    let _ = thread.join();
                                }
                            }
                            ClickStormMessage::Shutdown => {
                                // Shutdown the thread
                                println!("Shutting down click storm thread");
                                if let Some(thread) = thread.take() {
                                    is_working.store(false, Ordering::SeqCst);
                                    let _ = thread.join();
                                }
                                break;
                            }
                        }
                    }

                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }
        });

        Self {
            settings: AppSettings::new(),
            device_state: DeviceState::new(),
            display_size,
            picking_position: false,
            sender: Some(sender),
            is_running: Arc::new(AtomicBool::new(false)),
            key_pressed: false,
        }
    }
}

impl ClickStormApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ClickStormApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Send message to thread to stop click storm
        match self.sender.as_ref() {
            Some(sender) => {
                let _ = sender.send(ClickStormMessage::Shutdown);
            }
            None => {
                println!("Error sending message: Sender is None");
                // This should clean up the inner thread if it's still running
                self.is_running.store(false, Ordering::SeqCst);
            }
        }
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint so the input is updated
        // TODO: Move input to a separate thread and remove this
        ctx.request_repaint();

        // Handle input
        self.handle_input();

        // Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(self.get_locale_string("settings"), |ui| {
                    // Light/dark mode buttons
                    egui::widgets::global_dark_light_mode_buttons(ui);

                    ui.separator();

                    // Language selection
                    egui::ComboBox::from_label("")
                        .selected_text(self.settings.language().get_language().as_str())
                        .show_ui(ui, |ui| {
                            let mut lang = self.settings.language().get_language();
                            for language in Language::iter() {
                                let language_string = language.as_str();
                                ui.selectable_value(&mut lang, language.clone(), language_string);
                            }
                            self.settings.language_mut().set_language(lang);
                        });
                    ui.separator();

                    // About button
                    ui.menu_button(self.get_locale_string("about"), |ui| {
                        let version_label = format!(
                            "{}{}",
                            self.get_locale_string("version"),
                            env!("CARGO_PKG_VERSION")
                        );
                        ui.label(version_label);

                        ui.separator();

                        if ui
                            .hyperlink_to(
                                self.get_locale_string("source"),
                                "https://github.com/iliags/click_storm",
                            )
                            .clicked()
                        {
                            ui.close_menu();
                        }

                        #[cfg(debug_assertions)]
                        {
                            ui.separator();

                            egui::warn_if_debug_build(ui);
                        }
                    });
                });

                ui.separator();

                // Reset button
                if ui.button(self.get_locale_string("reset")).clicked() {
                    self.settings.reset();
                }

                #[cfg(debug_assertions)]
                {
                    ui.separator();

                    let doing_work = self.is_running.load(Ordering::SeqCst);
                    let message = format!("Working: {}", doing_work);
                    ui.label(message);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_interval(ui);

            ui.horizontal(|ui| {
                self.ui_click_options(ui);
                self.ui_click_repeat(ui);
            });

            self.ui_cursor_position(ui);
            self.ui_actions(ui);
        });
    }
}

impl ClickStormApp {
    fn handle_input(&mut self) {
        if self.picking_position {
            let mouse: MouseState = self.device_state.get_mouse();

            for press in mouse.button_pressed.iter() {
                if *press == true {
                    let coords = mouse.coords;
                    self.settings.cursor_position_fixed_mut().0 = coords.0;
                    self.settings.cursor_position_fixed_mut().1 = coords.1;
                    self.picking_position = false;
                    println!("Picked position: {:?}", coords);
                }
            }
        }

        // TODO: Custom key bindings
        let hot_key_pressed = self.device_state.get_keys().contains(&HOTKEY_CODE);

        if hot_key_pressed && !self.key_pressed {
            self.key_pressed = true;

            if self.is_running.load(Ordering::SeqCst) {
                //println!("Stop");
                self.stop_click_storm();
            } else {
                //println!("Start");
                self.start_click_storm();
            }
        } else if self.key_pressed && !hot_key_pressed {
            self.key_pressed = false;
        }
    }

    fn ui_interval(&mut self, ui: &mut egui::Ui) {
        let interval_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.heading(self.get_locale_string("click_interval"));
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
            });

        // Show the hover text for the interval frame
        interval_frame.response.on_hover_text(
            self.settings
                .language()
                .get_locale_string("click_interval_desc"),
        );
    }

    fn ui_click_options(&mut self, ui: &mut egui::Ui) {
        let click_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(self.get_locale_string("click_options"));

                    // Click button name
                    let selected_button = self.settings.mouse_button().as_str_locale().to_owned();

                    // Generate the combo box
                    egui::ComboBox::from_label(self.get_locale_string("mouse_button"))
                        .selected_text(self.get_locale_string(&selected_button))
                        .show_ui(ui, |ui| {
                            // Iterate over the click types
                            let mut current_value = self.settings.mouse_button().clone();
                            for mouse_button in MouseButton::iter() {
                                // Get the locale string for the click type
                                let mouse_button_locale = self
                                    .settings
                                    .language()
                                    .get_locale_string(mouse_button.as_str_locale());

                                // Select the click type
                                ui.selectable_value(
                                    &mut current_value,
                                    mouse_button,
                                    mouse_button_locale,
                                );
                            }
                            self.settings.mouse_button_mut().clone_from(&current_value);
                        });

                    // Click type options
                    // Get the selected click type name
                    let selected_click_type = self.settings.click_type().as_str_locale().to_owned();

                    // Generate the combo box
                    egui::ComboBox::from_label(self.get_locale_string("click_type"))
                        .selected_text(
                            self.settings
                                .language()
                                .get_locale_string(&selected_click_type),
                        )
                        .show_ui(ui, |ui| {
                            // Iterate over the click types
                            let mut current_value = self.settings.click_type().clone();
                            for click_type in MouseClickType::iter() {
                                // Get the locale string for the click type
                                let click_type_locale = self
                                    .settings
                                    .language()
                                    .get_locale_string(click_type.as_str_locale());

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

        click_frame.response.on_hover_text(
            self.settings
                .language()
                .get_locale_string("click_type_desc"),
        );
    }

    fn ui_click_repeat(&mut self, ui: &mut egui::Ui) {
        let repeat_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(self.get_locale_string("repeat_options"));

                    ui.horizontal(|ui| {
                        let repeat_count_name = self.get_locale_string("repeat_number");
                        let repeat_count = self.settings.repeat_count();
                        ui.radio_value(
                            self.settings.repeat_type_mut(),
                            RepeatType::Repeat(repeat_count),
                            repeat_count_name,
                        );

                        ui.add(
                            egui::DragValue::new(self.settings.repeat_count_mut())
                                .range(0..=1000)
                                .speed(1)
                                .clamp_to_range(false),
                        );
                    });

                    let repeat_infinite_name = self
                        .settings
                        .language()
                        .get_locale_string("repeat_until_stopped");
                    ui.radio_value(
                        self.settings.repeat_type_mut(),
                        RepeatType::RepeatUntilStopped,
                        repeat_infinite_name,
                    );
                });
            });

        repeat_frame
            .response
            .on_hover_text(self.get_locale_string("repeat_desc"));
    }

    fn ui_cursor_position(&mut self, ui: &mut egui::Ui) {
        let cursor_position_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.heading(
                    self.settings
                        .language()
                        .get_locale_string("cursor_position"),
                );

                ui.horizontal(|ui| {
                    ui.columns(5, |cols| {
                        cols[0].centered_and_justified(|ui| {
                            // Current position radio button
                            let current_position_name = self
                                .settings
                                .language()
                                .get_locale_string("current_position");

                            ui.radio_value(
                                self.settings.cursor_position_type_mut(),
                                CursorPosition::CurrentLocation,
                                current_position_name,
                            );
                        });
                        cols[1].centered_and_justified(|ui| {
                            // Fixed position radio button
                            let fixed_position_name = self.get_locale_string("fixed_position");
                            let current_position = self.settings.cursor_position_fixed();
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
                            if ui
                                .button(self.get_locale_string("pick_position"))
                                .on_hover_text_at_pointer(
                                    self.settings
                                        .language()
                                        .get_locale_string("pick_position_desc"),
                                )
                                .clicked()
                            {
                                // TODO: Add a visual cue that the user is picking a position
                                self.picking_position = true;
                            }
                        });
                        cols[3].centered_and_justified(|ui| {
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.settings.cursor_position_fixed_mut().0,
                                )
                                .range(0..=self.display_size.0)
                                .prefix("x: ")
                                .speed(1),
                            );
                        });
                        cols[4].centered_and_justified(|ui| {
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.settings.cursor_position_fixed_mut().1,
                                )
                                .range(0..=self.display_size.1)
                                .prefix("y: ")
                                .speed(1),
                            );
                        });
                    });
                });
            });

        cursor_position_frame
            .response
            .on_hover_text(self.settings.language().get_locale_string("position_desc"));
    }

    fn ui_actions(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        ui.centered_and_justified(|ui| {
            ui.columns(2, |cols| {
                let key_code_text = format!(" ({})", HOTKEY_CODE);
                cols[0].centered_and_justified(|ui| {
                    let start_text = self.get_locale_string("start") + &key_code_text;
                    if ui.button(start_text).clicked() {
                        self.start_click_storm();
                    }
                });
                cols[1].centered_and_justified(|ui| {
                    let stop_text = self.get_locale_string("stop") + &key_code_text;
                    if ui.button(stop_text).clicked() {
                        self.stop_click_storm();
                    }
                    ui.end_row();
                });
            });
        });
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.settings.language().get_locale_string(key)
    }

    fn start_click_storm(&mut self) {
        // Send message to thread to start click storm
        // Include a copy of the settings
        // Use a UI cue while the storm is running, maybe darken the UI

        if self.is_running.load(Ordering::SeqCst) {
            return;
        }

        match self.sender.as_ref() {
            Some(sender) => {
                let _ = sender.send(ClickStormMessage::Start(
                    self.settings.clone(),
                    Arc::clone(&self.is_running),
                ));
                self.is_running.store(true, Ordering::SeqCst);
            }
            None => {
                println!("Error sending message: Sender is None");
            }
        }
    }
    fn stop_click_storm(&mut self) {
        // Send message to thread to stop click storm

        match self.sender.as_ref() {
            Some(sender) => {
                let _ = sender.send(ClickStormMessage::Stop);
                self.is_running.store(false, Ordering::SeqCst);
            }
            None => {
                println!("Error sending message: Sender is None");
            }
        }
    }
}
