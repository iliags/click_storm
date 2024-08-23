use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum RepeatType {
    Repeat(usize),
    #[default]
    RepeatUntilStopped,
}

impl RepeatType {
    /// Get the key for the locale string
    pub fn as_str_locale(&self) -> &str {
        match self {
            Self::Repeat(_) => "repeat_number",
            Self::RepeatUntilStopped => "repeat_until_stopped",
        }
    }

    /// Get the number of repeats
    pub fn repeat_count(&self) -> Option<usize> {
        match self {
            Self::Repeat(count) => Some(*count),
            Self::RepeatUntilStopped => None,
        }
    }
}
