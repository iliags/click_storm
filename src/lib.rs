//! ClickStorm application library.

/// Application module
mod app;

/// Application settings
mod settings;

/// Localization module
mod localization;

/// Do once module
mod do_once;

/// Worker thread module
mod worker;

/// Input module
mod input;

/// Rhai scripting module
#[cfg(feature = "scripting")]
mod scripting;

/// Application module re-export
pub use app::ClickStormApp;
