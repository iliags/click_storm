use rhai::plugin::*;

#[derive(Debug, Clone)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for MousePosition {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[export_module]
#[allow(non_snake_case, dead_code)]
pub mod MouseModule {
    use super::*;

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(value: &mut MousePosition) -> String {
        format!("({}, {})", value.x, value.y)
    }

    #[rhai_fn(get = "x", pure)]
    pub fn x(value: &mut MousePosition) -> i32 {
        value.x
    }

    #[rhai_fn(get = "y", pure)]
    pub fn y(value: &mut MousePosition) -> i32 {
        value.y
    }
}
