use device_query::Keycode;
use enigo::Key;
use strum_macros::EnumIter;

// Re-export the `Keycode` enum from the `device_query` crate.
#[derive(
    Debug, Eq, PartialEq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize, EnumIter,
)]
#[allow(missing_docs)]
pub enum AppKeycode {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Command,
    LOption,
    ROption,
    LMeta,
    RMeta,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    CapsLock,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadSubtract,
    NumpadAdd,
    NumpadDivide,
    NumpadMultiply,
    NumpadEquals,
    NumpadEnter,
    NumpadDecimal,
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    BackSlash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,
}

impl AppKeycode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Key0 => "0",
            Self::Key1 => "1",
            Self::Key2 => "2",
            Self::Key3 => "3",
            Self::Key4 => "4",
            Self::Key5 => "5",
            Self::Key6 => "6",
            Self::Key7 => "7",
            Self::Key8 => "8",
            Self::Key9 => "9",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::Escape => "Escape",
            Self::Space => "Space",
            Self::LControl => "LControl",
            Self::RControl => "RControl",
            Self::LShift => "LShift",
            Self::RShift => "RShift",
            Self::LAlt => "LAlt",
            Self::RAlt => "RAlt",
            Self::Command => "Command",
            Self::LOption => "LOption",
            Self::ROption => "ROption",
            Self::LMeta => "LMeta",
            Self::RMeta => "RMeta",
            Self::Enter => "Enter",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Backspace => "Backspace",
            Self::CapsLock => "CapsLock",
            Self::Tab => "Tab",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PageUp",
            Self::PageDown => "PageDown",
            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::Numpad0 => "Numpad0",
            Self::Numpad1 => "Numpad1",
            Self::Numpad2 => "Numpad2",
            Self::Numpad3 => "Numpad3",
            Self::Numpad4 => "Numpad4",
            Self::Numpad5 => "Numpad5",
            Self::Numpad6 => "Numpad6",
            Self::Numpad7 => "Numpad7",
            Self::Numpad8 => "Numpad8",
            Self::Numpad9 => "Numpad9",
            Self::NumpadSubtract => "NumpadSubtract",
            Self::NumpadAdd => "NumpadAdd",
            Self::NumpadDivide => "NumpadDivide",
            Self::NumpadMultiply => "NumpadMultiply",
            Self::NumpadEquals => "NumpadEquals",
            Self::NumpadEnter => "NumpadEnter",
            Self::NumpadDecimal => "NumpadDecimal",
            Self::Grave => "Grave",
            Self::Minus => "Minus",
            Self::Equal => "Equal",
            Self::LeftBracket => "LeftBracket",
            Self::RightBracket => "RightBracket",
            Self::BackSlash => "BackSlash",
            Self::Semicolon => "Semicolon",
            Self::Apostrophe => "Apostrophe",
            Self::Comma => "Comma",
            Self::Dot => "Dot",
            Self::Slash => "Slash",
        }
    }
}

impl From<device_query::Keycode> for AppKeycode {
    fn from(value: Keycode) -> Self {
        match value {
            Keycode::Key0 => AppKeycode::Key0,
            Keycode::Key1 => AppKeycode::Key1,
            Keycode::Key2 => AppKeycode::Key2,
            Keycode::Key3 => AppKeycode::Key3,
            Keycode::Key4 => AppKeycode::Key4,
            Keycode::Key5 => AppKeycode::Key5,
            Keycode::Key6 => AppKeycode::Key6,
            Keycode::Key7 => AppKeycode::Key7,
            Keycode::Key8 => AppKeycode::Key8,
            Keycode::Key9 => AppKeycode::Key9,
            Keycode::A => AppKeycode::A,
            Keycode::B => AppKeycode::B,
            Keycode::C => AppKeycode::C,
            Keycode::D => AppKeycode::D,
            Keycode::E => AppKeycode::E,
            Keycode::F => AppKeycode::F,
            Keycode::G => AppKeycode::G,
            Keycode::H => AppKeycode::H,
            Keycode::I => AppKeycode::I,
            Keycode::J => AppKeycode::J,
            Keycode::K => AppKeycode::K,
            Keycode::L => AppKeycode::L,
            Keycode::M => AppKeycode::M,
            Keycode::N => AppKeycode::N,
            Keycode::O => AppKeycode::O,
            Keycode::P => AppKeycode::P,
            Keycode::Q => AppKeycode::Q,
            Keycode::R => AppKeycode::R,
            Keycode::S => AppKeycode::S,
            Keycode::T => AppKeycode::T,
            Keycode::U => AppKeycode::U,
            Keycode::V => AppKeycode::V,
            Keycode::W => AppKeycode::W,
            Keycode::X => AppKeycode::X,
            Keycode::Y => AppKeycode::Y,
            Keycode::Z => AppKeycode::Z,
            Keycode::F1 => AppKeycode::F1,
            Keycode::F2 => AppKeycode::F2,
            Keycode::F3 => AppKeycode::F3,
            Keycode::F4 => AppKeycode::F4,
            Keycode::F5 => AppKeycode::F5,
            Keycode::F6 => AppKeycode::F6,
            Keycode::F7 => AppKeycode::F7,
            Keycode::F8 => AppKeycode::F8,
            Keycode::F9 => AppKeycode::F9,
            Keycode::F10 => AppKeycode::F10,
            Keycode::F11 => AppKeycode::F11,
            Keycode::F12 => AppKeycode::F12,
            Keycode::F13 => AppKeycode::F13,
            Keycode::F14 => AppKeycode::F14,
            Keycode::F15 => AppKeycode::F15,
            Keycode::F16 => AppKeycode::F16,
            Keycode::F17 => AppKeycode::F17,
            Keycode::F18 => AppKeycode::F18,
            Keycode::F19 => AppKeycode::F19,
            Keycode::F20 => AppKeycode::F20,
            Keycode::Escape => AppKeycode::Escape,
            Keycode::Space => AppKeycode::Space,
            Keycode::LControl => AppKeycode::LControl,
            Keycode::RControl => AppKeycode::RControl,
            Keycode::LShift => AppKeycode::LShift,
            Keycode::RShift => AppKeycode::RShift,
            Keycode::LAlt => AppKeycode::LAlt,
            Keycode::RAlt => AppKeycode::RAlt,
            Keycode::Command => AppKeycode::Command,
            Keycode::LOption => AppKeycode::LOption,
            Keycode::ROption => AppKeycode::ROption,
            Keycode::LMeta => AppKeycode::LMeta,
            Keycode::RMeta => AppKeycode::RMeta,
            Keycode::Enter => AppKeycode::Enter,
            Keycode::Up => AppKeycode::Up,
            Keycode::Down => AppKeycode::Down,
            Keycode::Left => AppKeycode::Left,
            Keycode::Right => AppKeycode::Right,
            Keycode::Backspace => AppKeycode::Backspace,
            Keycode::CapsLock => AppKeycode::CapsLock,
            Keycode::Tab => AppKeycode::Tab,
            Keycode::Home => AppKeycode::Home,
            Keycode::End => AppKeycode::End,
            Keycode::PageUp => AppKeycode::PageUp,
            Keycode::PageDown => AppKeycode::PageDown,
            Keycode::Insert => AppKeycode::Insert,
            Keycode::Delete => AppKeycode::Delete,
            Keycode::Numpad0 => AppKeycode::Numpad0,
            Keycode::Numpad1 => AppKeycode::Numpad1,
            Keycode::Numpad2 => AppKeycode::Numpad2,
            Keycode::Numpad3 => AppKeycode::Numpad3,
            Keycode::Numpad4 => AppKeycode::Numpad4,
            Keycode::Numpad5 => AppKeycode::Numpad5,
            Keycode::Numpad6 => AppKeycode::Numpad6,
            Keycode::Numpad7 => AppKeycode::Numpad7,
            Keycode::Numpad8 => AppKeycode::Numpad8,
            Keycode::Numpad9 => AppKeycode::Numpad9,
            Keycode::NumpadSubtract => AppKeycode::NumpadSubtract,
            Keycode::NumpadAdd => AppKeycode::NumpadAdd,
            Keycode::NumpadDivide => AppKeycode::NumpadDivide,
            Keycode::NumpadMultiply => AppKeycode::NumpadMultiply,
            Keycode::NumpadEquals => AppKeycode::NumpadEquals,
            Keycode::NumpadEnter => AppKeycode::NumpadEnter,
            Keycode::NumpadDecimal => AppKeycode::NumpadDecimal,
            Keycode::Grave => AppKeycode::Grave,
            Keycode::Minus => AppKeycode::Minus,
            Keycode::Equal => AppKeycode::Equal,
            Keycode::LeftBracket => AppKeycode::LeftBracket,
            Keycode::RightBracket => AppKeycode::RightBracket,
            Keycode::BackSlash => AppKeycode::BackSlash,
            Keycode::Semicolon => AppKeycode::Semicolon,
            Keycode::Apostrophe => AppKeycode::Apostrophe,
            Keycode::Comma => AppKeycode::Comma,
            Keycode::Dot => AppKeycode::Dot,
            Keycode::Slash => AppKeycode::Slash,
        }
    }
}

impl From<AppKeycode> for device_query::Keycode {
    fn from(val: AppKeycode) -> Self {
        match val {
            AppKeycode::Key0 => Keycode::Key0,
            AppKeycode::Key1 => Keycode::Key1,
            AppKeycode::Key2 => Keycode::Key2,
            AppKeycode::Key3 => Keycode::Key3,
            AppKeycode::Key4 => Keycode::Key4,
            AppKeycode::Key5 => Keycode::Key5,
            AppKeycode::Key6 => Keycode::Key6,
            AppKeycode::Key7 => Keycode::Key7,
            AppKeycode::Key8 => Keycode::Key8,
            AppKeycode::Key9 => Keycode::Key9,
            AppKeycode::A => Keycode::A,
            AppKeycode::B => Keycode::B,
            AppKeycode::C => Keycode::C,
            AppKeycode::D => Keycode::D,
            AppKeycode::E => Keycode::E,
            AppKeycode::F => Keycode::F,
            AppKeycode::G => Keycode::G,
            AppKeycode::H => Keycode::H,
            AppKeycode::I => Keycode::I,
            AppKeycode::J => Keycode::J,
            AppKeycode::K => Keycode::K,
            AppKeycode::L => Keycode::L,
            AppKeycode::M => Keycode::M,
            AppKeycode::N => Keycode::N,
            AppKeycode::O => Keycode::O,
            AppKeycode::P => Keycode::P,
            AppKeycode::Q => Keycode::Q,
            AppKeycode::R => Keycode::R,
            AppKeycode::S => Keycode::S,
            AppKeycode::T => Keycode::T,
            AppKeycode::U => Keycode::U,
            AppKeycode::V => Keycode::V,
            AppKeycode::W => Keycode::W,
            AppKeycode::X => Keycode::X,
            AppKeycode::Y => Keycode::Y,
            AppKeycode::Z => Keycode::Z,
            AppKeycode::F1 => Keycode::F1,
            AppKeycode::F2 => Keycode::F2,
            AppKeycode::F3 => Keycode::F3,
            AppKeycode::F4 => Keycode::F4,
            AppKeycode::F5 => Keycode::F5,
            AppKeycode::F6 => Keycode::F6,
            AppKeycode::F7 => Keycode::F7,
            AppKeycode::F8 => Keycode::F8,
            AppKeycode::F9 => Keycode::F9,
            AppKeycode::F10 => Keycode::F10,
            AppKeycode::F11 => Keycode::F11,
            AppKeycode::F12 => Keycode::F12,
            AppKeycode::F13 => Keycode::F13,
            AppKeycode::F14 => Keycode::F14,
            AppKeycode::F15 => Keycode::F15,
            AppKeycode::F16 => Keycode::F16,
            AppKeycode::F17 => Keycode::F17,
            AppKeycode::F18 => Keycode::F18,
            AppKeycode::F19 => Keycode::F19,
            AppKeycode::F20 => Keycode::F20,
            AppKeycode::Escape => Keycode::Escape,
            AppKeycode::Space => Keycode::Space,
            AppKeycode::LControl => Keycode::LControl,
            AppKeycode::RControl => Keycode::RControl,
            AppKeycode::LShift => Keycode::LShift,
            AppKeycode::RShift => Keycode::RShift,
            AppKeycode::LAlt => Keycode::LAlt,
            AppKeycode::RAlt => Keycode::RAlt,
            AppKeycode::Command => Keycode::Command,
            AppKeycode::LOption => Keycode::LOption,
            AppKeycode::ROption => Keycode::ROption,
            AppKeycode::LMeta => Keycode::LMeta,
            AppKeycode::RMeta => Keycode::RMeta,
            AppKeycode::Enter => Keycode::Enter,
            AppKeycode::Up => Keycode::Up,
            AppKeycode::Down => Keycode::Down,
            AppKeycode::Left => Keycode::Left,
            AppKeycode::Right => Keycode::Right,
            AppKeycode::Backspace => Keycode::Backspace,
            AppKeycode::CapsLock => Keycode::CapsLock,
            AppKeycode::Tab => Keycode::Tab,
            AppKeycode::Home => Keycode::Home,
            AppKeycode::End => Keycode::End,
            AppKeycode::PageUp => Keycode::PageUp,
            AppKeycode::PageDown => Keycode::PageDown,
            AppKeycode::Insert => Keycode::Insert,
            AppKeycode::Delete => Keycode::Delete,
            AppKeycode::Numpad0 => Keycode::Numpad0,
            AppKeycode::Numpad1 => Keycode::Numpad1,
            AppKeycode::Numpad2 => Keycode::Numpad2,
            AppKeycode::Numpad3 => Keycode::Numpad3,
            AppKeycode::Numpad4 => Keycode::Numpad4,
            AppKeycode::Numpad5 => Keycode::Numpad5,
            AppKeycode::Numpad6 => Keycode::Numpad6,
            AppKeycode::Numpad7 => Keycode::Numpad7,
            AppKeycode::Numpad8 => Keycode::Numpad8,
            AppKeycode::Numpad9 => Keycode::Numpad9,
            AppKeycode::NumpadSubtract => Keycode::NumpadSubtract,
            AppKeycode::NumpadAdd => Keycode::NumpadAdd,
            AppKeycode::NumpadDivide => Keycode::NumpadDivide,
            AppKeycode::NumpadMultiply => Keycode::NumpadMultiply,
            AppKeycode::NumpadEquals => Keycode::NumpadEquals,
            AppKeycode::NumpadEnter => Keycode::NumpadEnter,
            AppKeycode::NumpadDecimal => Keycode::NumpadDecimal,
            AppKeycode::Grave => Keycode::Grave,
            AppKeycode::Minus => Keycode::Minus,
            AppKeycode::Equal => Keycode::Equal,
            AppKeycode::LeftBracket => Keycode::LeftBracket,
            AppKeycode::RightBracket => Keycode::RightBracket,
            AppKeycode::BackSlash => Keycode::BackSlash,
            AppKeycode::Semicolon => Keycode::Semicolon,
            AppKeycode::Apostrophe => Keycode::Apostrophe,
            AppKeycode::Comma => Keycode::Comma,
            AppKeycode::Dot => Keycode::Dot,
            AppKeycode::Slash => Keycode::Slash,
        }
    }
}

impl From<enigo::Key> for AppKeycode {
    fn from(value: enigo::Key) -> Self {
        match value {
            Key::Num0 => AppKeycode::Key0,
            Key::Num1 => AppKeycode::Key1,
            Key::Num2 => AppKeycode::Key2,
            Key::Num3 => AppKeycode::Key3,
            Key::Num4 => AppKeycode::Key4,
            Key::Num5 => AppKeycode::Key5,
            Key::Num6 => AppKeycode::Key6,
            Key::Num7 => AppKeycode::Key7,
            Key::Num8 => AppKeycode::Key8,
            Key::Num9 => AppKeycode::Key9,
            _ => todo!(),
        }
    }
}

#[cfg(feature = "scripting")]
use rhai::plugin::*;

#[cfg(feature = "scripting")]
#[cfg_attr(feature = "scripting", export_module)]
#[allow(non_snake_case, non_upper_case_globals, dead_code)]
pub mod MouseButtonModule {
    use super::*;

    pub const Key0: AppKeycode = AppKeycode::Key0;
    pub const Key1: AppKeycode = AppKeycode::Key1;
    pub const Key2: AppKeycode = AppKeycode::Key2;
    pub const Key3: AppKeycode = AppKeycode::Key3;
    pub const Key4: AppKeycode = AppKeycode::Key4;
    pub const Key5: AppKeycode = AppKeycode::Key5;
    pub const Key6: AppKeycode = AppKeycode::Key6;
    pub const Key7: AppKeycode = AppKeycode::Key7;
    pub const Key8: AppKeycode = AppKeycode::Key8;
    pub const Key9: AppKeycode = AppKeycode::Key9;
    pub const A: AppKeycode = AppKeycode::A;
    pub const B: AppKeycode = AppKeycode::B;
    pub const C: AppKeycode = AppKeycode::C;
    pub const D: AppKeycode = AppKeycode::D;
    pub const E: AppKeycode = AppKeycode::E;
    pub const F: AppKeycode = AppKeycode::F;
    pub const G: AppKeycode = AppKeycode::G;
    pub const H: AppKeycode = AppKeycode::H;
    pub const I: AppKeycode = AppKeycode::I;
    pub const J: AppKeycode = AppKeycode::J;
    pub const K: AppKeycode = AppKeycode::K;
    pub const L: AppKeycode = AppKeycode::L;
    pub const M: AppKeycode = AppKeycode::M;
    pub const N: AppKeycode = AppKeycode::N;
    pub const O: AppKeycode = AppKeycode::O;
    pub const P: AppKeycode = AppKeycode::P;
    pub const Q: AppKeycode = AppKeycode::Q;
    pub const R: AppKeycode = AppKeycode::R;
    pub const S: AppKeycode = AppKeycode::S;
    pub const T: AppKeycode = AppKeycode::T;
    pub const U: AppKeycode = AppKeycode::U;
    pub const V: AppKeycode = AppKeycode::V;
    pub const W: AppKeycode = AppKeycode::W;
    pub const X: AppKeycode = AppKeycode::X;
    pub const Y: AppKeycode = AppKeycode::Y;
    pub const Z: AppKeycode = AppKeycode::Z;
    pub const F1: AppKeycode = AppKeycode::F1;
    pub const F2: AppKeycode = AppKeycode::F2;
    pub const F3: AppKeycode = AppKeycode::F3;
    pub const F4: AppKeycode = AppKeycode::F4;
    pub const F5: AppKeycode = AppKeycode::F5;
    pub const F6: AppKeycode = AppKeycode::F6;
    pub const F7: AppKeycode = AppKeycode::F7;
    pub const F8: AppKeycode = AppKeycode::F8;
    pub const F9: AppKeycode = AppKeycode::F9;
    pub const F10: AppKeycode = AppKeycode::F10;
    pub const F11: AppKeycode = AppKeycode::F11;
    pub const F12: AppKeycode = AppKeycode::F12;
    pub const F13: AppKeycode = AppKeycode::F13;
    pub const F14: AppKeycode = AppKeycode::F14;
    pub const F15: AppKeycode = AppKeycode::F15;
    pub const F16: AppKeycode = AppKeycode::F16;
    pub const F17: AppKeycode = AppKeycode::F17;
    pub const F18: AppKeycode = AppKeycode::F18;
    pub const F19: AppKeycode = AppKeycode::F19;
    pub const F20: AppKeycode = AppKeycode::F20;
    pub const Escape: AppKeycode = AppKeycode::Escape;
    pub const Space: AppKeycode = AppKeycode::Space;
    pub const LControl: AppKeycode = AppKeycode::LControl;
    pub const RControl: AppKeycode = AppKeycode::RControl;
    pub const LShift: AppKeycode = AppKeycode::LShift;
    pub const RShift: AppKeycode = AppKeycode::RShift;
    pub const LAlt: AppKeycode = AppKeycode::LAlt;
    pub const RAlt: AppKeycode = AppKeycode::RAlt;
    pub const Command: AppKeycode = AppKeycode::Command;
    pub const LOption: AppKeycode = AppKeycode::LOption;
    pub const ROption: AppKeycode = AppKeycode::ROption;
    pub const LMeta: AppKeycode = AppKeycode::LMeta;
    pub const RMeta: AppKeycode = AppKeycode::RMeta;
    pub const Enter: AppKeycode = AppKeycode::Enter;
    pub const Up: AppKeycode = AppKeycode::Up;
    pub const Down: AppKeycode = AppKeycode::Down;
    pub const Left: AppKeycode = AppKeycode::Left;
    pub const Right: AppKeycode = AppKeycode::Right;
    pub const Backspace: AppKeycode = AppKeycode::Backspace;
    pub const CapsLock: AppKeycode = AppKeycode::CapsLock;
    pub const Tab: AppKeycode = AppKeycode::Tab;
    pub const Home: AppKeycode = AppKeycode::Home;
    pub const End: AppKeycode = AppKeycode::End;
    pub const PageUp: AppKeycode = AppKeycode::PageUp;
    pub const PageDown: AppKeycode = AppKeycode::PageDown;
    pub const Insert: AppKeycode = AppKeycode::Insert;
    pub const Delete: AppKeycode = AppKeycode::Delete;
    pub const Numpad0: AppKeycode = AppKeycode::Numpad0;
    pub const Numpad1: AppKeycode = AppKeycode::Numpad1;
    pub const Numpad2: AppKeycode = AppKeycode::Numpad2;
    pub const Numpad3: AppKeycode = AppKeycode::Numpad3;
    pub const Numpad4: AppKeycode = AppKeycode::Numpad4;
    pub const Numpad5: AppKeycode = AppKeycode::Numpad5;
    pub const Numpad6: AppKeycode = AppKeycode::Numpad6;
    pub const Numpad7: AppKeycode = AppKeycode::Numpad7;
    pub const Numpad8: AppKeycode = AppKeycode::Numpad8;
    pub const Numpad9: AppKeycode = AppKeycode::Numpad9;
    pub const NumpadSubtract: AppKeycode = AppKeycode::NumpadSubtract;
    pub const NumpadAdd: AppKeycode = AppKeycode::NumpadAdd;
    pub const NumpadDivide: AppKeycode = AppKeycode::NumpadDivide;
    pub const NumpadMultiply: AppKeycode = AppKeycode::NumpadMultiply;
    pub const NumpadEquals: AppKeycode = AppKeycode::NumpadEquals;
    pub const NumpadEnter: AppKeycode = AppKeycode::NumpadEnter;
    pub const NumpadDecimal: AppKeycode = AppKeycode::NumpadDecimal;
    pub const Grave: AppKeycode = AppKeycode::Grave;
    pub const Minus: AppKeycode = AppKeycode::Minus;
    pub const Equal: AppKeycode = AppKeycode::Equal;
    pub const LeftBracket: AppKeycode = AppKeycode::LeftBracket;
    pub const RightBracket: AppKeycode = AppKeycode::RightBracket;
    pub const BackSlash: AppKeycode = AppKeycode::BackSlash;
    pub const Semicolon: AppKeycode = AppKeycode::Semicolon;
    pub const Apostrophe: AppKeycode = AppKeycode::Apostrophe;
    pub const Comma: AppKeycode = AppKeycode::Comma;
    pub const Dot: AppKeycode = AppKeycode::Dot;
    pub const Slash: AppKeycode = AppKeycode::Slash;

    /// Return the current variant of `MouseButton`.
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut AppKeycode) -> String {
        value.as_str().to_string()
    }

    #[rhai_fn(global, get = "value", pure)]
    pub fn get_value(_: &mut AppKeycode) -> Dynamic {
        Dynamic::UNIT
    }

    // Printing
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(value: &mut AppKeycode) -> String {
        format!("{value:?}")
    }

    // '==' and '!=' operators
    #[rhai_fn(global, name = "==", pure)]
    pub fn eq(lhs: &mut AppKeycode, rhs: AppKeycode) -> bool {
        lhs == &rhs
    }
    #[rhai_fn(global, name = "!=", pure)]
    pub fn neq(lhs: &mut AppKeycode, rhs: AppKeycode) -> bool {
        lhs != &rhs
    }
}
