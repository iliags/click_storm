use cs_hal::input::button_direction::ButtonDirection;
use rhai::plugin::*;

#[export_module]
#[allow(non_snake_case, non_upper_case_globals)]
pub mod ButtonDirectionModule {

    pub const Press: ButtonDirection = ButtonDirection::Press;
    pub const Release: ButtonDirection = ButtonDirection::Release;
    pub const Click: ButtonDirection = ButtonDirection::Click;

    /// Return the current variant of `ButtonDirection`.
    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut ButtonDirection) -> String {
        value.as_str().to_string()
    }

    /// Return the value of `ButtonDirection`.
    #[rhai_fn(global, get = "value", pure)]
    pub fn get_value(_: &mut ButtonDirection) -> Dynamic {
        Dynamic::UNIT
    }

    // Printing
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(value: &mut ButtonDirection) -> String {
        format!("{value:?}")
    }

    // '==' and '!=' operators
    #[rhai_fn(global, name = "==", pure)]
    pub fn eq(lhs: &mut ButtonDirection, rhs: ButtonDirection) -> bool {
        lhs == &rhs
    }
    #[rhai_fn(global, name = "!=", pure)]
    pub fn neq(lhs: &mut ButtonDirection, rhs: ButtonDirection) -> bool {
        lhs != &rhs
    }
}
