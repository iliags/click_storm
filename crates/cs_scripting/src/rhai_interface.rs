use std::sync::{Arc, Mutex};

use cs_hal::{
    display::screen_size::{ScreenSize, ScreenSizeModule},
    input::{
        button_direction::{ButtonDirection, ButtonDirectionModule},
        keycode::{AppKeycode, AppKeycodeModule},
        mouse_button::{MouseButton, MouseButtonModule},
    },
};
use rhai::{exported_module, Engine};

use super::cs_interface::ClickStormInterface;

/// The Rhai scripting interface
#[derive(Debug, Clone)]
pub struct RhaiInterface {
    // Note: Using Arc because this might be shared between threads, not sure yet
    engine: Arc<Mutex<Engine>>,
}

impl RhaiInterface {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            engine: Arc::new(Mutex::new(Engine::new())),
        }
    }

    /// Test hello world
    #[cfg(debug_assertions)]
    pub fn test_hello(&mut self) {
        let engine = self.engine.lock().unwrap();

        engine.run(r#"print("hello, world!")"#).unwrap();

        let result = engine.eval::<i32>("40 + 2").unwrap();
        println!("Result: {}", result);
    }

    /// Run the test script
    #[cfg(debug_assertions)]
    pub fn test_script(&mut self) {
        let engine = self.engine.lock().unwrap();

        let test_script = include_str!("../../../scripts/test.rhai");
        engine.run(test_script).unwrap();
    }

    /// Get the engine instance
    pub fn get_engine(&self) -> Arc<Mutex<Engine>> {
        self.engine.clone()
    }

    /// Initialize the Rhai engine with the necessary functions and types.
    pub fn initialize(&mut self) {
        let mut engine = self.engine.lock().unwrap();

        // Register the scripting interface
        engine.register_type::<RhaiInterface>();
        engine.register_type::<ClickStormInterface>();

        // Register the mouse button enum
        engine
            .register_type_with_name::<MouseButton>("MouseButton")
            .register_static_module("MouseButton", exported_module!(MouseButtonModule).into());

        // Register screen size
        engine
            .register_type_with_name::<ScreenSize>("ScreenSize")
            .register_static_module("ScreenSize", exported_module!(ScreenSizeModule).into());

        // Register keycodes
        engine
            .register_type_with_name::<AppKeycode>("AppKeycode")
            .register_static_module("AppKeycode", exported_module!(AppKeycodeModule).into());

        // Register button direction
        engine
            .register_type_with_name::<ButtonDirection>("ButtonDirection")
            .register_static_module(
                "ButtonDirection",
                exported_module!(ButtonDirectionModule).into(),
            );

        // Register the functions
        engine.register_fn("new_click_storm", ClickStormInterface::new);

        // TODO: Add duration/lerp and key chord versions
        engine
            .register_fn("click_at", ClickStormInterface::click_at)
            .register_fn("click_within", ClickStormInterface::click_within)
            .register_fn("move_mouse", ClickStormInterface::move_mouse_to)
            .register_fn("add_position", ClickStormInterface::add_position)
            .register_fn("drag_to", ClickStormInterface::drag_to)
            .register_fn("drag_from_to", ClickStormInterface::drag_from_to);

        engine.register_fn("set_key", ClickStormInterface::set_key);

        engine.register_fn("get_screen_size", ClickStormInterface::get_screen_size);

        engine
            .register_fn("rand_range", ClickStormInterface::rand_range)
            .register_fn("rand_range_excl", ClickStormInterface::rand_range_excl)
            .register_fn("rand_bool", ClickStormInterface::rand_bool);
    }
}
