use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum CursorPosition {
    #[default]
    CurrentLocation,
    FixedLocation(i32, i32),
}

impl CursorPosition {
    pub fn position(&self) -> Option<(i32, i32)> {
        match self {
            Self::CurrentLocation => None,
            Self::FixedLocation(x, y) => Some((*x, *y)),
        }
    }
}
