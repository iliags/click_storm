use std::sync::{Arc, Mutex};

use cs_hal::{
    display::screen_size::ScreenSize,
    input::{button_direction::ButtonDirection, keycode::AppKeycode, mouse_button::MouseButton},
};
use rhai::{exported_module, Engine};

use crate::{
    hal::{
        button_direction::ButtonDirectionModule,
        keycode::AppKeycodeModule,
        mouse::{MouseModule, MousePosition},
        mouse_button::MouseButtonModule,
        screen_size::ScreenSizeModule,
    },
    output_log::OutputLog,
};

use super::cs_interface::ClickStormInterface;

/// The test script
pub const TEST_SCRIPT: &str = include_str!("../../../scripts/test.rhai");

/// The Rhai scripting interface
#[derive(Debug)]
pub struct RhaiInterface {
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
        let mut new_self = Self {
            engine: Engine::new(),
        };

        // Initialize the engine with necessary functions and types
        new_self.initialize();

        new_self
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
    pub fn run_script(
        &mut self,
        script: &str,
        output_log: Arc<Mutex<OutputLog>>,
    ) -> Result<(), String> {
        let output_log = output_log.clone();
        self.engine.on_print(move |msg| {
            output_log.lock().unwrap().log(msg);
        });

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
    fn initialize(&mut self) {
        // Register the scripting interface
        self.engine.register_type::<ClickStormInterface>();

        // Register the mouse button enum
        self.engine
            .register_type_with_name::<MouseButton>("MouseButton")
            .register_static_module("MouseButton", exported_module!(MouseButtonModule).into());

        // Register the mouse position type
        self.engine
            .register_type_with_name::<MousePosition>("MouseButton")
            .register_static_module("MouseButton", exported_module!(MouseModule).into());

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
            .register_fn("new_click_storm", ClickStormInterface::new);

        // Click and drag related
        self.engine
            .register_fn("click_at", ClickStormInterface::click_at)
            .register_fn("click_within", ClickStormInterface::click_within)
            .register_fn("move_mouse_to", ClickStormInterface::move_mouse_to)
            .register_fn(
                "move_mouse_to",
                ClickStormInterface::move_mouse_to_screen_size,
            )
            .register_fn("add_position", ClickStormInterface::add_position)
            .register_fn("scroll", ClickStormInterface::scroll)
            .register_fn(
                "get_mouse_position",
                ClickStormInterface::get_mouse_position,
            )
            .register_fn("drag_to", ClickStormInterface::drag_to)
            .register_fn("drag_from_to", ClickStormInterface::drag_from_to)
            .register_fn("drag_from_to_rel", ClickStormInterface::drag_from_to_rel);

        // Keyboard related
        self.engine
            .register_fn("set_key", ClickStormInterface::set_key);

        // Screen size related
        self.engine
            .register_fn("get_screen_size", ClickStormInterface::get_screen_size);

        // Random number related
        // TODO: Move these to a separate module
        self.engine
            .register_fn("rand_range", ClickStormInterface::rand_range)
            .register_fn("rand_range_excl", ClickStormInterface::rand_range_excl)
            .register_fn("rand_bool", ClickStormInterface::rand_bool)
            .register_fn("rand_bool", ClickStormInterface::rand_bool_prob);
    }
}
