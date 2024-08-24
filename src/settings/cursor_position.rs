use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum CursorPosition {
    #[default]
    CurrentLocation,
    FixedLocation(i32, i32),
}
