use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, EnumIter)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,

    Back,
    Forward,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

impl MouseButton {
    /// Get the key for the locale string
    pub fn as_str_locale(&self) -> &str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Middle => "middle",
            Self::Back => "back",
            Self::Forward => "forward",
            Self::ScrollUp => "scroll_up",
            Self::ScrollDown => "scroll_down",
            Self::ScrollLeft => "scroll_left",
            Self::ScrollRight => "scroll_right",
        }
    }

    /// Get the string representation of the enum, used for scripting
    pub fn as_str(&self) -> &str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Middle => "Middle",
            Self::Back => "Back",
            Self::Forward => "Forward",
            Self::ScrollUp => "ScrollUp",
            Self::ScrollDown => "ScrollDown",
            Self::ScrollLeft => "ScrollLeft",
            Self::ScrollRight => "ScrollRight",
        }
    }
}

impl From<enigo::Button> for MouseButton {
    fn from(button: enigo::Button) -> Self {
        match button {
            enigo::Button::Left => Self::Left,
            enigo::Button::Right => Self::Right,
            enigo::Button::Middle => Self::Middle,
            enigo::Button::Back => Self::Back,
            enigo::Button::Forward => Self::Forward,
            enigo::Button::ScrollUp => Self::ScrollUp,
            enigo::Button::ScrollDown => Self::ScrollDown,
            enigo::Button::ScrollLeft => Self::ScrollLeft,
            enigo::Button::ScrollRight => Self::ScrollRight,
        }
    }
}

impl From<&MouseButton> for enigo::Button {
    fn from(button: &MouseButton) -> Self {
        match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
            MouseButton::Back => enigo::Button::Back,
            MouseButton::Forward => enigo::Button::Forward,
            MouseButton::ScrollUp => enigo::Button::ScrollUp,
            MouseButton::ScrollDown => enigo::Button::ScrollDown,
            MouseButton::ScrollLeft => enigo::Button::ScrollLeft,
            MouseButton::ScrollRight => enigo::Button::ScrollRight,
        }
    }
}

impl From<MouseButton> for enigo::Button {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
            MouseButton::Back => enigo::Button::Back,
            MouseButton::Forward => enigo::Button::Forward,
            MouseButton::ScrollUp => enigo::Button::ScrollUp,
            MouseButton::ScrollDown => enigo::Button::ScrollDown,
            MouseButton::ScrollLeft => enigo::Button::ScrollLeft,
            MouseButton::ScrollRight => enigo::Button::ScrollRight,
        }
    }
}

#[cfg(feature = "scripting")]
use rhai::plugin::*;

#[cfg(feature = "scripting")]
#[cfg_attr(feature = "scripting", export_module)]
#[allow(non_snake_case)]
pub mod MouseButtonModule {
    use super::*;

    #[allow(non_upper_case_globals)]
    pub const Left: MouseButton = MouseButton::Left;

    #[allow(non_upper_case_globals)]
    pub const Right: MouseButton = MouseButton::Right;

    #[allow(non_upper_case_globals)]
    pub const Middle: MouseButton = MouseButton::Middle;

    /// Return the current variant of `MouseButton`.
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(button_enum: &mut MouseButton) -> String {
        button_enum.as_str().to_string()
    }

    #[rhai_fn(global, get = "value", pure)]
    pub fn get_value(_: &mut MouseButton) -> Dynamic {
        Dynamic::UNIT
    }

    // Printing
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(button_enum: &mut MouseButton) -> String {
        format!("{button_enum:?}")
    }

    // '==' and '!=' operators
    #[rhai_fn(global, name = "==", pure)]
    pub fn eq(lhs: &mut MouseButton, rhs: MouseButton) -> bool {
        lhs == &rhs
    }
    #[rhai_fn(global, name = "!=", pure)]
    pub fn neq(lhs: &mut MouseButton, rhs: MouseButton) -> bool {
        lhs != &rhs
    }
}
