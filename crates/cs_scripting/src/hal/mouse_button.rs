use cs_hal::input::mouse_button::MouseButton;
use rhai::plugin::*;

#[export_module]
#[allow(non_snake_case, non_upper_case_globals)]
pub mod MouseButtonModule {

    pub const Left: MouseButton = MouseButton::Left;
    pub const Right: MouseButton = MouseButton::Right;
    pub const Middle: MouseButton = MouseButton::Middle;
    pub const ScrollUp: MouseButton = MouseButton::ScrollUp;
    pub const ScrollDown: MouseButton = MouseButton::ScrollDown;
    pub const ScrollLeft: MouseButton = MouseButton::ScrollLeft;
    pub const ScrollRight: MouseButton = MouseButton::ScrollRight;

    /// Return the current variant of `MouseButton`.
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut MouseButton) -> String {
        value.as_str().to_string()
    }

    /// Return the value of `MouseButton`.
    #[rhai_fn(global, get = "value", pure)]
    pub fn get_value(_: &mut MouseButton) -> Dynamic {
        Dynamic::UNIT
    }

    // Printing
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(value: &mut MouseButton) -> String {
        format!("{value:?}")
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
