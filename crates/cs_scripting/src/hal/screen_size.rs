use cs_hal::display::screen_size::ScreenSize;
use rhai::plugin::{
    export_module, Dynamic, FnNamespace, FuncRegistration, Module, NativeCallContext, PluginFunc,
    RhaiResult, TypeId,
};

#[export_module]
#[allow(non_snake_case, dead_code)]
pub mod ScreenSizeModule {

    #[rhai_fn(global, pure)]
    pub fn to_debug(object: &mut ScreenSize) -> String {
        format!("({}, {})", object.x(), object.y())
    }

    #[rhai_fn(global, pure)]
    pub fn to_string(object: &mut ScreenSize) -> String {
        format!("{}x{}", object.x(), object.y())
    }

    /// Get the width of the screen
    #[rhai_fn(get = "width", pure)]
    pub fn width(object: &mut ScreenSize) -> i32 {
        object.x()
    }

    /// Get the height of the screen
    #[rhai_fn(get = "height", pure)]
    pub fn height(object: &mut ScreenSize) -> i32 {
        object.y()
    }

    #[rhai_fn(get = "x", pure)]
    pub fn x(object: &mut ScreenSize) -> i32 {
        object.x()
    }

    #[rhai_fn(get = "y", pure)]
    pub fn y(object: &mut ScreenSize) -> i32 {
        object.y()
    }

    /// Get the center coordinates of the screen
    #[rhai_fn(global, pure)]
    pub fn center(object: &mut ScreenSize) -> ScreenSize {
        object.center()
    }
}
