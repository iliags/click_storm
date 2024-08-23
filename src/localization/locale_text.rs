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

    /// Get the current language mutable
    pub fn get_language_mut(&mut self) -> &mut Language {
        &mut self.language
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
