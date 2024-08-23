use std::time::Duration;

use strum_macros::EnumIter;

use crate::localization::locale_text::LocaleText;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum MouseClickType {
    #[default]
    Single,
    Double,
}

//Note: Maybe add a direction for which way the mouse is clicking.
// Instead of just clicking down, it could be holding down, and the "click" could be a release.

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppSettings {
    // Language
    language: LocaleText,

    // Interval components
    interval_hours: usize,
    interval_minutes: usize,
    interval_seconds: usize,
    interval_milliseconds: usize,

    // Mouse click settings
    mouse_button: MouseButton,
    mouse_click_type: MouseClickType,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            interval_milliseconds: 100,
            ..Default::default()
        }
    }

    /// Get the click interval duration
    pub fn click_interval(&self) -> Duration {
        Duration::from_secs(
            (self.interval_hours as u64) * 3600
                + (self.interval_minutes as u64) * 60
                + self.interval_seconds as u64
                + (self.interval_milliseconds as u64) / 1000,
        )
    }

    /// Get the current language
    pub fn language(&self) -> &LocaleText {
        &self.language
    }

    /// Get a mutable reference to the current language
    pub fn language_mut(&mut self) -> &mut LocaleText {
        &mut self.language
    }

    pub fn interval_hours(&self) -> usize {
        self.interval_hours
    }

    pub fn interval_hours_mut(&mut self) -> &mut usize {
        &mut self.interval_hours
    }

    pub fn interval_minutes(&self) -> usize {
        self.interval_minutes
    }

    pub fn interval_minutes_mut(&mut self) -> &mut usize {
        &mut self.interval_minutes
    }

    pub fn interval_seconds(&self) -> usize {
        self.interval_seconds
    }

    pub fn interval_seconds_mut(&mut self) -> &mut usize {
        &mut self.interval_seconds
    }

    pub fn interval_milliseconds(&self) -> usize {
        self.interval_milliseconds
    }

    pub fn interval_milliseconds_mut(&mut self) -> &mut usize {
        &mut self.interval_milliseconds
    }
}
