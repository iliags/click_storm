use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum RepeatType {
    Repeat(usize),
    #[default]
    RepeatUntilStopped,
}
