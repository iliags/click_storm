use egui::Margin;
use strum::IntoEnumIterator;

use crate::settings::{
    app_settings::AppSettings, mouse_button::MouseButton, mouse_click::MouseClickType,
    repeat::RepeatType,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ClickStormApp {
    settings: AppSettings,
}

impl Default for ClickStormApp {
    fn default() -> Self {
        Self {
            settings: AppSettings::new(),
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

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.menu_button(self.settings.language().get_locale_string("about"), |ui| {
                        let version_label = format!(
                            "{}{}",
                            self.settings.language().get_locale_string("version"),
                            env!("CARGO_PKG_VERSION")
                        );
                        ui.label(version_label);

                        ui.separator();

                        ui.hyperlink_to(
                            self.settings.language().get_locale_string("source"),
                            "https://github.com/iliags/click_storm",
                        );

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
                ui.heading(self.settings.language().get_locale_string("click_interval"));
                ui.horizontal(|ui| {
                    // Hours
                    ui.label(self.settings.language().get_locale_string("hours"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_hours_mut())
                            .range(0..=24)
                            .speed(1),
                    );

                    ui.separator();

                    // Minutes
                    ui.label(self.settings.language().get_locale_string("minutes"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_minutes_mut())
                            .range(0..=60)
                            .speed(1),
                    );

                    ui.separator();

                    // Seconds
                    ui.label(self.settings.language().get_locale_string("seconds"));
                    ui.add(
                        egui::DragValue::new(self.settings.interval_seconds_mut())
                            .range(0..=60)
                            .speed(1),
                    );

                    ui.separator();

                    // Milliseconds
                    ui.label(self.settings.language().get_locale_string("milliseconds"));
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
                    ui.heading(self.settings.language().get_locale_string("click_options"));

                    // Click button name
                    let selected_button = self.settings.mouse_button().as_str_locale().to_owned();

                    // Generate the combo box
                    egui::ComboBox::from_label(
                        self.settings.language().get_locale_string("mouse_button"),
                    )
                    .selected_text(self.settings.language().get_locale_string(&selected_button))
                    .show_ui(ui, |ui| {
                        // Iterate over the click types
                        for mut mouse_button in MouseButton::iter() {
                            // Get the locale string for the click type
                            let mouse_button_locale = self
                                .settings
                                .language()
                                .get_locale_string(mouse_button.as_str_locale());

                            // Select the click type
                            ui.selectable_value(
                                &mut self.settings.mouse_button_mut(),
                                &mut mouse_button,
                                mouse_button_locale,
                            );
                        }
                    });

                    // Click type options
                    // Get the selected click type name
                    let selected_click_type = self.settings.click_type().as_str_locale().to_owned();

                    // Generate the combo box
                    egui::ComboBox::from_label(
                        self.settings.language().get_locale_string("click_type"),
                    )
                    .selected_text(
                        self.settings
                            .language()
                            .get_locale_string(&selected_click_type),
                    )
                    .show_ui(ui, |ui| {
                        // Iterate over the click types
                        for mut click_type in MouseClickType::iter() {
                            // Get the locale string for the click type
                            let click_type_locale = self
                                .settings
                                .language()
                                .get_locale_string(click_type.as_str_locale());

                            // Select the click type
                            ui.selectable_value(
                                &mut self.settings.click_type_mut(),
                                &mut click_type,
                                click_type_locale,
                            );
                        }
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
                    ui.heading(self.settings.language().get_locale_string("repeat_options"));

                    ui.horizontal(|ui| {
                        let repeat_count_name =
                            self.settings.language().get_locale_string("repeat_number");
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
            .on_hover_text(self.settings.language().get_locale_string("repeat_desc"));
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
                ui.horizontal(|ui| {});
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
            if ui
                .button(self.settings.language().get_locale_string("start"))
                .clicked()
            {
                self.start_click_storm();
            }
            if ui
                .button(self.settings.language().get_locale_string("stop"))
                .clicked()
            {
                self.stop_click_storm();
            }
            ui.end_row();
        });
    }

    fn start_click_storm(&mut self) {}
    fn stop_click_storm(&mut self) {}
}
