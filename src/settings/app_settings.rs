use std::time::Duration;

use cs_hal::input::{mouse_button::MouseButton, mouse_click::MouseClickType};

use super::{cursor_position::CursorPosition, repeat_type::RepeatType};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppSettings {
    // Interval components
    interval_hours: usize,
    interval_minutes: usize,
    interval_seconds: usize,
    interval_milliseconds: usize,

    // Mouse click settings
    mouse_button: MouseButton,
    mouse_click_type: MouseClickType,

    // Repeat settings
    repeat_type: RepeatType,

    // Cursor position
    cursor_position_type: CursorPosition,

    repeat_variation: usize,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            interval_milliseconds: 100,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get the click interval duration
    pub fn click_interval(&self) -> Duration {
        if self.interval_hours > 0 {
            Duration::from_secs(
                (self.interval_hours as u64) * 3600
                    + (self.interval_minutes as u64) * 60
                    + self.interval_seconds as u64
                    + (self.interval_milliseconds as u64) / 1000,
            )
        } else if self.interval_minutes > 0 {
            Duration::from_secs(
                (self.interval_minutes as u64) * 60
                    + self.interval_seconds as u64
                    + (self.interval_milliseconds as u64) / 1000,
            )
        } else if self.interval_seconds > 0 {
            Duration::from_secs(
                self.interval_seconds as u64 + (self.interval_milliseconds as u64) / 1000,
            )
        } else {
            Duration::from_millis(self.interval_milliseconds as u64)
        }
    }

    pub fn click_interval_milliseconds(&self) -> u64 {
        self.interval_milliseconds as u64
    }

    pub fn interval_hours_mut(&mut self) -> &mut usize {
        &mut self.interval_hours
    }

    pub fn interval_minutes_mut(&mut self) -> &mut usize {
        &mut self.interval_minutes
    }

    pub fn interval_seconds_mut(&mut self) -> &mut usize {
        &mut self.interval_seconds
    }

    pub fn interval_milliseconds_mut(&mut self) -> &mut usize {
        &mut self.interval_milliseconds
    }

    pub fn click_type(&self) -> &MouseClickType {
        &self.mouse_click_type
    }

    pub fn click_type_mut(&mut self) -> &mut MouseClickType {
        &mut self.mouse_click_type
    }

    pub fn mouse_button(&self) -> &MouseButton {
        &self.mouse_button
    }

    pub fn mouse_button_mut(&mut self) -> &mut MouseButton {
        &mut self.mouse_button
    }

    pub fn repeat_type(&self) -> &RepeatType {
        &self.repeat_type
    }

    pub fn repeat_type_mut(&mut self) -> &mut RepeatType {
        &mut self.repeat_type
    }

    pub fn set_repeat_type(&mut self, repeat_type: RepeatType) {
        self.repeat_type = repeat_type;
    }

    pub fn cursor_position_type(&self) -> &CursorPosition {
        &self.cursor_position_type
    }

    pub fn cursor_position_type_mut(&mut self) -> &mut CursorPosition {
        &mut self.cursor_position_type
    }

    pub fn set_cursor_position_type(&mut self, cursor_position_type: CursorPosition) {
        self.cursor_position_type = cursor_position_type;
    }

    pub fn repeat_variation(&self) -> &usize {
        &self.repeat_variation
    }

    pub fn repeat_variation_mut(&mut self) -> &mut usize {
        &mut self.repeat_variation
    }
}
