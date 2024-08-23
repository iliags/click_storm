use strum_macros::EnumIter;

//Note: Maybe add a direction for which way the mouse is clicking.
// Instead of just clicking down, it could be holding down, and the "click" could be a release.

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum MouseClickType {
    #[default]
    Single,
    Double,
}

// TODO: Add localization for the mouse button and click type
impl MouseClickType {
    /// Get the string representation of the click type
    pub fn as_str(&self) -> &str {
        match self {
            Self::Single => "Click",
            Self::Double => "Double Click",
        }
    }

    /// Get the key for the locale string
    pub fn as_str_locale(&self) -> &str {
        match self {
            Self::Single => "single_click",
            Self::Double => "double_click",
        }
    }
}
