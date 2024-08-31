#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonDirection {
    Press,
    Release,
    #[default]
    Click,
}

impl ButtonDirection {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Press => "Press",
            Self::Release => "Release",
            Self::Click => "Click",
        }
    }
}

impl From<enigo::Direction> for ButtonDirection {
    fn from(direction: enigo::Direction) -> Self {
        match direction {
            enigo::Direction::Press => Self::Press,
            enigo::Direction::Release => Self::Release,
            enigo::Direction::Click => Self::Click,
        }
    }
}

impl From<ButtonDirection> for enigo::Direction {
    fn from(direction: ButtonDirection) -> Self {
        match direction {
            ButtonDirection::Press => enigo::Direction::Press,
            ButtonDirection::Release => enigo::Direction::Release,
            ButtonDirection::Click => enigo::Direction::Click,
        }
    }
}
