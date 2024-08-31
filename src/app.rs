use cs_hal::input::keycode::AppKeycode;
use device_query::{DeviceQuery, DeviceState};
use strum::IntoEnumIterator;

use crate::do_once::DoOnceGate;
use crate::localization::language::Language;
use crate::localization::locale_text::LocaleText;
use crate::ui::clicker::{self, ClickerPanel};
use crate::ui::UIPanel;

#[cfg(feature = "scripting")]
use crate::ui::script::{self, ScriptPanel};

// Wishlist:
// - Record and playback mouse movements
// - Check github for updates
//  - Use GET /repos/:owner/:repo/releases/latest
// - Write a lint or comp-time check to find unused/mismatched keys in the localization files
// - Disable widgets that are not applicable when certain settings are selected

/// Application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ClickStormApp {
    hotkey_code: AppKeycode,

    language: LocaleText,

    active_panel: usize,

    #[serde(skip)]
    panels: Vec<Box<dyn UIPanel>>,

    #[serde(skip)]
    device_state: DeviceState,

    #[serde(skip)]
    wait_for_key: DoOnceGate,

    #[serde(skip)]
    key_pressed: bool,
}

impl Default for ClickStormApp {
    fn default() -> Self {
        let panels: Vec<Box<dyn UIPanel>> = vec![
            Box::new(ClickerPanel::default()),
            #[cfg(feature = "scripting")]
            Box::new(ScriptPanel::default()),
        ];

        Self {
            hotkey_code: AppKeycode::F6,
            language: LocaleText::default(),
            active_panel: 0,
            panels,
            device_state: DeviceState::new(),
            wait_for_key: DoOnceGate::default(),
            key_pressed: false,
        }
    }
}

impl eframe::App for ClickStormApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);

        // TODO: Find an easier way to save the panel state
        for panel in self.panels.iter_mut() {
            if panel.as_any().downcast_ref::<ClickerPanel>().is_some() {
                let clicker_panel = panel.as_any().downcast_ref::<ClickerPanel>().unwrap();
                eframe::set_value(storage, clicker::CLICKER_PANEL_KEY, clicker_panel);
            }

            #[cfg(feature = "scripting")]
            if panel.as_any().downcast_ref::<ScriptPanel>().is_some() {
                let script_panel = panel.as_any().downcast_ref::<ScriptPanel>().unwrap();
                eframe::set_value(storage, script::SCRIPT_PANEL_KEY, script_panel);
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        for panel in self.panels.iter_mut() {
            panel.exit();
        }
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint so the input is updated
        ctx.request_repaint();

        // Handle input
        self.handle_input();
        self.panels[self.active_panel].handle_input();

        // Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("⛭", |ui| {
                    // Change hotkey button
                    ui.label(self.get_locale_string("hotkey"));
                    let button_text = if self.wait_for_key.is_active() {
                        self.get_locale_string("press_key")
                    } else {
                        self.get_locale_string("change_hotkey")
                    };
                    if ui
                        .button(button_text)
                        .on_hover_text_at_pointer(self.get_locale_string("change_hotkey_desc"))
                        .clicked()
                    {
                        self.wait_for_key.set_active();
                    }

                    if ui
                        .button(self.get_locale_string("reset_hotkey"))
                        .on_hover_text_at_pointer(self.get_locale_string("reset_hotkey_desc"))
                        .clicked()
                    {
                        self.hotkey_code = AppKeycode::F6;
                        ui.close_menu();
                    }

                    for panel in self.panels.iter_mut() {
                        panel.show_settings(ctx, ui);
                    }

                    ui.separator();

                    // Language selection
                    ui.label(self.get_locale_string("language"));

                    egui::ComboBox::from_label("")
                        .selected_text(self.language.get_language().as_str())
                        .show_ui(ui, |ui| {
                            let mut lang = self.language.get_language();
                            for language in Language::iter() {
                                let language_string = language.as_str();
                                ui.selectable_value(&mut lang, language.clone(), language_string);
                            }
                            self.language.set_language(lang);

                            for panel in self.panels.iter_mut() {
                                panel.set_language(self.language.clone());
                            }
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

                    ui.separator();

                    egui::widgets::global_dark_light_mode_buttons(ui);
                });

                ui.separator();

                // Reset button
                if ui
                    .button("⟳")
                    .on_hover_text_at_pointer(self.get_locale_string("reset"))
                    .clicked()
                {
                    self.panels[self.active_panel].reset();
                }

                ui.separator();

                #[cfg(feature = "scripting")]
                {
                    for (index, panel) in self.panels.iter().enumerate() {
                        let panel_name = panel.name();

                        ui.radio_value(&mut self.active_panel, index, panel_name);
                    }

                    ui.separator();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.panels[self.active_panel].show(ctx, ui);
        });
    }
}

impl ClickStormApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            let mut value: ClickStormApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            // Make sure the active panel is within bounds
            if value.active_panel >= value.panels.len() {
                value.active_panel = 0;
            }

            // Load the panel state
            // TODO Find a better way to load the panel state
            for panel in value.panels.iter_mut() {
                if panel.as_any().downcast_ref::<ClickerPanel>().is_some() {
                    let clicker_panel: ClickerPanel =
                        eframe::get_value(storage, clicker::CLICKER_PANEL_KEY).unwrap_or_default();
                    panel
                        .as_any()
                        .downcast_mut::<ClickerPanel>()
                        .unwrap()
                        .load(clicker_panel);
                }

                // Note: I apparently can't use cfg on an else if
                #[cfg(feature = "scripting")]
                if panel.as_any().downcast_ref::<ScriptPanel>().is_some() {
                    let script_panel: ScriptPanel =
                        eframe::get_value(storage, script::SCRIPT_PANEL_KEY).unwrap_or_default();
                    panel
                        .as_any()
                        .downcast_mut::<ScriptPanel>()
                        .unwrap()
                        .load(script_panel);
                }
            }

            return value;
        }

        Default::default()
    }

    fn handle_input(&mut self) {
        if self.wait_for_key.is_active() {
            let keys = self.device_state.get_keys();

            // Get the first key pressed
            if !keys.is_empty() {
                self.wait_for_key.set_waiting();

                self.hotkey_code = AppKeycode::from(keys[0]);
                println!("Hotkey set to: {:?}", self.hotkey_code);

                for panel in self.panels.iter_mut() {
                    panel.set_hotkey(self.hotkey_code);
                }
            }
        } else if self.wait_for_key.is_waiting_for_reset() {
            let keys = self.device_state.get_keys();

            if keys.is_empty() {
                self.wait_for_key.reset();
            }
        } else {
            let hot_key_pressed = self
                .device_state
                .get_keys()
                .contains(&self.hotkey_code.into());

            if hot_key_pressed && !self.key_pressed {
                self.key_pressed = true;
                self.panels[self.active_panel].toggle();
            } else if self.key_pressed && !hot_key_pressed {
                self.key_pressed = false;
            }
        }
    }

    #[inline]
    fn get_locale_string(&self, key: &str) -> String {
        self.language.get_locale_string(key)
    }
}
