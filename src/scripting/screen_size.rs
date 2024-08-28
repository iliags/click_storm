use rhai::plugin::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ScreenSize {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for ScreenSize {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl ScreenSize {
    pub fn center(&self) -> ScreenSize {
        (self.x / 2, self.y / 2).into()
    }
}

#[cfg(feature = "scripting")]
#[cfg_attr(feature = "scripting", export_module)]
#[allow(non_snake_case, dead_code)]
pub mod ScreenSizeModule {
    use super::*;

    #[rhai_fn(global, pure)]
    pub fn to_debug(object: &mut ScreenSize) -> String {
        format!("({}, {})", object.x, object.y)
    }

    #[rhai_fn(global, pure)]
    pub fn to_string(object: &mut ScreenSize) -> String {
        format!("{}x{}", object.x, object.y)
    }

    #[rhai_fn(get = "width", pure)]
    pub fn get_width(object: &mut ScreenSize) -> i32 {
        object.x
    }

    #[rhai_fn(get = "height", pure)]
    pub fn get_height(object: &mut ScreenSize) -> i32 {
        object.y
    }

    #[rhai_fn(global, pure)]
    pub fn center(object: &mut ScreenSize) -> ScreenSize {
        object.center()
    }
}