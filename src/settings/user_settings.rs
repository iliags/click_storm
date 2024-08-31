use cs_hal::input::keycode::AppKeycode;

use crate::localization::locale_text::LocaleText;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug, Clone)]
pub struct UserSettings {
    hotkey: AppKeycode,

    language: LocaleText,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            hotkey: AppKeycode::F6,
            language: LocaleText::default(),
        }
    }
}

impl UserSettings {
    pub fn hotkey(&self) -> AppKeycode {
        self.hotkey
    }

    pub fn set_hotkey(&mut self, hotkey: AppKeycode) {
        self.hotkey = hotkey;
    }

    pub fn reset_hotkey(&mut self) {
        self.hotkey = AppKeycode::F6;
    }

    pub fn language(&self) -> &LocaleText {
        &self.language
    }

    pub fn language_mut(&mut self) -> &mut LocaleText {
        &mut self.language
    }
}
