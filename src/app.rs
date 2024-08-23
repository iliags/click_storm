use device_query::{DeviceQuery, DeviceState, MouseState};
use egui::Margin;
use enigo::{Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use strum::IntoEnumIterator;

use crate::{
    localization::language::Language,
    settings::{
        app_settings::AppSettings, cursor_position::CursorPosition, mouse_button::MouseButton,
        mouse_click::MouseClickType, repeat::RepeatType,
    },
};

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
    is_running: bool,
}

#[derive(Debug, Clone)]
enum ClickStormMessage {
    Start(AppSettings),
    Stop,
    Shutdown,
}

impl Default for ClickStormApp {
    fn default() -> Self {
        // TODO: Handle error
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        let display_size = enigo.main_display().unwrap();

        let (sender, receiver): (Sender<ClickStormMessage>, Receiver<ClickStormMessage>) =
            channel();

        thread::spawn(move || {
            let mut thread: Option<JoinHandle<()>> = None;
            let mut settings_clone = Arc::new(AppSettings::default());
            let stop_flag = Arc::new(AtomicBool::new(false));

            loop {
                match receiver.recv() {
                    Ok(message) => match message {
                        ClickStormMessage::Start(settings) => {
                            // Start the click storm
                            println!("Starting click storm with settings: {:?}", settings);

                            if let Some(thread) = thread.take() {
                                stop_flag.store(true, Ordering::SeqCst);
                                let _ = thread.join();
                            }

                            let stop_flag_clone = Arc::clone(&stop_flag);
                            stop_flag.store(false, Ordering::SeqCst);

                            let new_settings = Arc::new(settings);
                            settings_clone = Arc::clone(&new_settings);

                            // Worker thread
                            thread = Some(thread::spawn(move || {
                                let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
                                    panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
                                });
                                while stop_flag_clone.load(Ordering::SeqCst) == false {
                                    println!(
                                        "Inner thread performing task with settings: {:?}",
                                        settings_clone
                                    );
                                    thread::sleep(Duration::from_secs(1));
                                }
                            }));
                        }
                        ClickStormMessage::Stop => {
                            // Stop the click storm
                            println!("Stopping click storm");
                            if let Some(thread) = thread.take() {
                                stop_flag.store(true, Ordering::SeqCst);
                                let _ = thread.join();
                            }
                        }
                        ClickStormMessage::Shutdown => {
                            // Shutdown the thread
                            println!("Shutting down click storm thread");
                            if let Some(thread) = thread.take() {
                                stop_flag.store(true, Ordering::SeqCst);
                                let _ = thread.join();
                            }
                            break;
                        }
                    },

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
            display_size: display_size,
            picking_position: false,
            sender: Some(sender),
            is_running: false,
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
            }
        }
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        ctx.input(|i| {
            if i.key_pressed(egui::Key::F6) {
                println!("F6 pressed");
                // TODO: Toggle start/stop click storm
                // TODO: Custom key bindings
            }
        });

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

                            //println!("Window size: {:?}", ctx.screen_rect());

                            egui::warn_if_debug_build(ui);
                        }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //println!("Window size: {:?}", ctx.screen_rect());
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
    fn ui_interval(&mut self, ui: &mut egui::Ui) {
        let interval_frame = egui::Frame::default()
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
            .rounding(ui.visuals().widgets.noninteractive.rounding)
            .inner_margin(Margin::same(4.0))
            .show(ui, |ui| {
                ui.heading(self.get_locale_string("click_interval"));
                ui.horizontal(|ui| {
                    // Hours
                    ui.label(self.get_locale_string("hours"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_hours_mut())
                            .range(0..=24)
                            .speed(1),
                    );

                    ui.separator();

                    // Minutes
                    ui.label(self.get_locale_string("minutes"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_minutes_mut())
                            .range(0..=60)
                            .speed(1),
                    );

                    ui.separator();

                    // Seconds
                    ui.label(self.get_locale_string("seconds"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_seconds_mut())
                            .range(0..=60)
                            .speed(1),
                    );

                    ui.separator();

                    // Milliseconds
                    ui.label(self.get_locale_string("milliseconds"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_milliseconds_mut())
                            .range(0..=1000)
                            .speed(1),
                    );
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

                    // Fixed position radio button
                    let fixed_position_name = self.get_locale_string("fixed_position");
                    let current_position = self.settings.cursor_position_fixed();
                    ui.radio_value(
                        self.settings.cursor_position_type_mut(),
                        CursorPosition::FixedLocation(current_position.0, current_position.1),
                        fixed_position_name,
                    );

                    if ui
                        .button(self.get_locale_string("pick_position"))
                        .on_hover_text_at_pointer(
                            self.settings
                                .language()
                                .get_locale_string("pick_position_desc"),
                        )
                        .clicked()
                    {
                        self.picking_position = true;
                        // TODO: Add a visual cue that the user is picking a position
                    }

                    ui.add(
                        egui::DragValue::new(&mut self.settings.cursor_position_fixed_mut().0)
                            .range(0..=self.display_size.0)
                            .prefix("x: ")
                            .speed(1),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.settings.cursor_position_fixed_mut().1)
                            .range(0..=self.display_size.1)
                            .prefix("y: ")
                            .speed(1),
                    );
                });
            });

        cursor_position_frame.response.on_hover_text(
            self.settings
                .language()
                .get_locale_string("pick_position_desc"),
        );
    }

    fn ui_actions(&mut self, ui: &mut egui::Ui) {
        // TODO: Center the buttons
        egui::Grid::new("actions").show(ui, |ui| {
            if ui.button(self.get_locale_string("start")).clicked() {
                self.start_click_storm();
            }
            if ui.button(self.get_locale_string("stop")).clicked() {
                self.stop_click_storm();
            }
            ui.end_row();
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

        if self.is_running {
            return;
        }

        match self.sender.as_ref() {
            Some(sender) => {
                let _ = sender.send(ClickStormMessage::Start(self.settings.clone()));
                self.is_running = true;
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
                self.is_running = false;
            }
            None => {
                println!("Error sending message: Sender is None");
            }
        }
    }
}
