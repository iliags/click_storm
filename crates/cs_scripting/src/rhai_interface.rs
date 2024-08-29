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

/// The test script
pub const TEST_SCRIPT: &str = include_str!("../../../scripts/test.rhai");

/// The Rhai scripting interface
#[derive(Debug)]
pub struct RhaiInterface {
    // Note: Using Arc because this might be shared between threads, not sure yet
    engine: Engine,
}

impl Default for RhaiInterface {
    fn default() -> Self {
        Self::new()
    }
}

impl RhaiInterface {
    /// Create a new engine instance
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    /// Test hello world
    #[cfg(debug_assertions)]
    pub fn test_hello(&mut self) {
        self.engine.run(r#"print("hello, world!")"#).unwrap();

        let result = self.engine.eval::<i32>("40 + 2").unwrap();
        println!("Result: {}", result);
    }

    /// Run the test script
    #[cfg(debug_assertions)]
    pub fn test_script(&mut self) {
        self.engine.run(TEST_SCRIPT).unwrap();
    }

    /// Run a script
    pub fn run_script(&mut self, script: &str) -> Result<(), String> {
        // TODO: Propogate errors
        match self.engine.run(script) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error: {}", err);
                Err(err.to_string())
            }
        }
    }

    /// Get the engine instance (only for codegen)
    pub fn get_engine(&self) -> &Engine {
        println!("This should only be used for codegen");
        &self.engine
    }

    /// Initialize the Rhai engine with the necessary functions and types.
    pub fn initialize(&mut self) {
        // Register the scripting interface
        self.engine.register_type::<ClickStormInterface>();

        // Register the mouse button enum
        self.engine
            .register_type_with_name::<MouseButton>("MouseButton")
            .register_static_module("MouseButton", exported_module!(MouseButtonModule).into());

        // Register screen size
        self.engine
            .register_type_with_name::<ScreenSize>("ScreenSize")
            .register_static_module("ScreenSize", exported_module!(ScreenSizeModule).into());

        // Register keycodes
        self.engine
            .register_type_with_name::<AppKeycode>("AppKeycode")
            .register_static_module("AppKeycode", exported_module!(AppKeycodeModule).into());

        // Register button direction
        self.engine
            .register_type_with_name::<ButtonDirection>("ButtonDirection")
            .register_static_module(
                "ButtonDirection",
                exported_module!(ButtonDirectionModule).into(),
            );

        // Register the functions
        self.engine
            .register_fn("click_storm", ClickStormInterface::new);

        // Click and drag related
        self.engine
            .register_fn("click_at", ClickStormInterface::click_at)
            .register_fn("click_within", ClickStormInterface::click_within)
            .register_fn("move_mouse", ClickStormInterface::move_mouse_to)
            .register_fn("add_position", ClickStormInterface::add_position)
            .register_fn("drag_to", ClickStormInterface::drag_to)
            .register_fn("drag_from_to", ClickStormInterface::drag_from_to);

        // Keyboard related
        self.engine
            .register_fn("set_key", ClickStormInterface::set_key);

        // Screen size related
        self.engine
            .register_fn("get_screen_size", ClickStormInterface::get_screen_size);

        // Random number related
        self.engine
            .register_fn("rand_range", ClickStormInterface::rand_range)
            .register_fn("rand_range_excl", ClickStormInterface::rand_range_excl)
            .register_fn("rand_bool", ClickStormInterface::rand_bool);
    }
}
