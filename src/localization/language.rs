// The static loader macro doesn't work with documentation comments, so we have to disable the warning.
#![allow(missing_docs)]
use fluent_templates::static_loader;
use strum_macros::EnumIter;
use unic_langid::{langid, LanguageIdentifier};

// TODO: French, German, and Spain Spanish

/// The languages available for localization
#[derive(Default, Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, EnumIter)]
pub enum Language {
    #[default]
    English,
}

impl Language {
    // Note: Make sure the return value is a valid language identifier
    pub fn value(&self) -> LanguageIdentifier {
        match self {
            Self::English => langid!("en-US"),
        }
    }

    // Note: Make sure the text matches the native form (i.e. "Français" for French)
    pub fn as_str(&self) -> &str {
        match self {
            Self::English => "English",
        }
    }
}

#[cfg(debug_assertions)]
static_loader! {
    pub static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
        // Should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[cfg(not(debug_assertions))]
static_loader! {
    pub static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
    };
}
