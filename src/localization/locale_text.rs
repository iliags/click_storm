use fluent_templates::Loader;

use super::language::{Language, LOCALES};

/// Struct to hold the current locale of the application.
#[derive(Default, Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct LocaleText {
    language: Language,
}

impl LocaleText {
    /// Get the current language
    pub fn get_language(&self) -> Language {
        self.language.clone()
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    /// Get a string from the current locale with the given key
    pub fn get_locale_string(&self, key: &str) -> String {
        LOCALES.lookup(&self.language.value(), key)
    }
}
