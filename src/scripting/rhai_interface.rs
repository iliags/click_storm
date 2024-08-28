#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use device_query::DeviceState;
use enigo::{Button, Enigo, Keyboard, Mouse, Settings};
use rand::Rng;
use rhai::{exported_module, Engine};

use crate::input::{
    button_direction::{ButtonDirection, ButtonDirectionModule},
    keycode::{AppKeycode, AppKeycodeModule},
    mouse_button::{MouseButton, MouseButtonModule},
};

use super::screen_size::{ScreenSize, ScreenSizeModule};

// TODO: Export metadata for the scripting API to use in a language server
// See: https://rhai.rs/book/engine/metadata/definitions.html

// TODO: Find a way to exclude the evaluation engine from the data passed to the script
#[derive(Debug, Clone)]
pub struct RhaiInterface {
    engine: Arc<Mutex<Engine>>,
    enigo: Arc<Mutex<Enigo>>,
    device: DeviceState,
    rng: rand::rngs::ThreadRng,
    screen_size: Option<ScreenSize>,
}

impl RhaiInterface {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).unwrap_or_else(|_| {
            panic!("Failed to create Enigo instance. Please make sure you are running the application on a system that supports the Enigo library.")
        });

        Self {
            engine: Arc::new(Mutex::new(Engine::new())),
            enigo: Arc::new(Mutex::new(enigo)),
            device: DeviceState::new(),
            rng: rand::thread_rng(),
            screen_size: None,
        }
    }

    #[cfg(debug_assertions)]
    pub fn test_hello(&mut self) {
        let engine = self.engine.lock().unwrap();

        engine.run(r#"print("hello, world!")"#).unwrap();

        let result = engine.eval::<i64>("40 + 2").unwrap();
        println!("Result: {}", result);
    }

    #[cfg(debug_assertions)]
    pub fn test_script(&mut self) {
        let engine = self.engine.lock().unwrap();

        let test_script = include_str!("../../scripts/test.rhai");
        engine.run(test_script).unwrap();
    }

    // TODO: Move this to an external binary
    #[cfg(debug_assertions)]
    pub fn generate_definitions(&mut self) {
        let engine = self.engine.lock().unwrap();

        //let definitions = engine.generate_metadata().unwrap();
        //println!("{}", definitions);
        engine
            .definitions()
            .with_headers(true)
            .include_standard_packages(false)
            .write_to_file("D:/click_storm/scripts/click_storm_api.d.rhai")
            .unwrap();
    }

    /// Initialize the Rhai engine with the necessary functions and types.
    pub fn initialize(&mut self) {
        let mut engine = self.engine.lock().unwrap();

        // Register the scripting interface
        engine.register_type::<RhaiInterface>();

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
        engine.register_fn("new_engine", RhaiInterface::new);

        // TODO: Add duration/lerp and key chord versions
        engine
            .register_fn("click_at", RhaiInterface::click_at)
            .register_fn("move_mouse", RhaiInterface::move_mouse_to)
            .register_fn("add_position", RhaiInterface::add_position)
            .register_fn("drag_to", RhaiInterface::drag_to)
            .register_fn("drag_from_to", RhaiInterface::drag_from_to);

        engine.register_fn("set_key", RhaiInterface::set_key);

        engine.register_fn("get_screen_size", RhaiInterface::get_screen_size);

        engine
            .register_fn("rand_range", RhaiInterface::rand_range)
            .register_fn("rand_range_excl", RhaiInterface::rand_range_excl)
            .register_fn("rand_bool", RhaiInterface::rand_bool);
    }

    /**************************************************************************
     * Scripting API
     **************************************************************************/
    // Note: I am pretty sure calling an API function from another API function will cause issues

    /// Click at the specified coordinates with the specified mouse button.
    fn click_at(&mut self, x: i32, y: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        let _ = enigo.button(mouse_button.into(), enigo::Direction::Click);
    }

    // Click within the specified specified area, X/Y starts at the top left corner.
    fn click_within(&mut self, x: i32, y: i32, width: i32, height: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();

        let rand_coords = (
            self.rng.gen_range(x..=x + width),
            self.rng.gen_range(y..=y + height),
        );

        let _ = enigo.move_mouse(rand_coords.0, rand_coords.1, enigo::Coordinate::Abs);
        let _ = enigo.button(mouse_button.into(), enigo::Direction::Click);
    }

    /// Move the mouse to the specified coordinates.
    fn move_mouse_to(&mut self, x: i32, y: i32) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
    }

    /// Adds the coordinates to the current mouse position.
    fn add_position(&mut self, x: i32, y: i32) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Rel);
    }

    /// Drags from the current mouse position to the specified coordinates.
    fn drag_to(&mut self, x: i32, y: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let button: Button = mouse_button.clone().into();
        let _ = enigo.button(button, enigo::Direction::Press);
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Release);
    }

    /// Drags from the specified coordinates to the specified coordinates.
    fn drag_from_to(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let button: Button = mouse_button.clone().into();
        let _ = enigo.move_mouse(x1, y1, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Press);
        let _ = enigo.move_mouse(x2, y2, enigo::Coordinate::Abs);
        let _ = enigo.button(button, enigo::Direction::Release);
    }

    fn set_key(&mut self, key: AppKeycode, direction: ButtonDirection) {
        let mut enigo = self.enigo.lock().unwrap();

        let _ = enigo.key(key.into(), direction.into());
    }

    /// Get the screen size.
    fn get_screen_size(&mut self) -> ScreenSize {
        match &self.screen_size {
            Some(size) => size.clone(),
            None => {
                let enigo = self.enigo.lock().unwrap();
                let screen_size = enigo
                    .main_display()
                    .unwrap_or_else(|_| panic!("Failed to get screen size."));

                let size: ScreenSize = screen_size.into();
                self.screen_size = Some(size.clone());

                size.clone()
            }
        }
    }

    /// Get a random number within the specified range (min inclusive, max inclusive).
    fn rand_range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..=max)
    }

    /// Get a random number within the specified range (min inclusive, max exclusive).
    fn rand_range_excl(&mut self, min: i32, max: i32) -> i32 {
        self.rng.gen_range(min..max)
    }

    /// Get a random boolean value (50/50).
    fn rand_bool(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }

    /**************************************************************************
     * End of Scripting API
     **************************************************************************/
}
