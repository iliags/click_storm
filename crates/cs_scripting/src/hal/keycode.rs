use cs_hal::input::keycode::AppKeycode;
use rhai::plugin::{
    export_module, mem, Dynamic, FnNamespace, FuncRegistration, Module, NativeCallContext,
    PluginFunc, RhaiResult, TypeId,
};

#[export_module]
#[allow(non_snake_case, non_upper_case_globals, dead_code)]
pub mod AppKeycodeModule {

    // Note: This is required to expose the `AppKeycode` enum to the scripting environment.
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
    pub const Control: AppKeycode = AppKeycode::LControl;
    pub const LControl: AppKeycode = AppKeycode::LControl;
    pub const RControl: AppKeycode = AppKeycode::RControl;
    pub const Shift: AppKeycode = AppKeycode::LShift;
    pub const LShift: AppKeycode = AppKeycode::LShift;
    pub const RShift: AppKeycode = AppKeycode::RShift;
    pub const Alt: AppKeycode = AppKeycode::LAlt;
    pub const LAlt: AppKeycode = AppKeycode::LAlt;
    pub const RAlt: AppKeycode = AppKeycode::RAlt;
    pub const Command: AppKeycode = AppKeycode::Command;
    pub const Option: AppKeycode = AppKeycode::LOption;
    pub const LOption: AppKeycode = AppKeycode::LOption;
    pub const ROption: AppKeycode = AppKeycode::ROption;
    pub const Meta: AppKeycode = AppKeycode::LMeta;
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
