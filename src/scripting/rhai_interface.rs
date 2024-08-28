#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use device_query::DeviceState;
use enigo::{Button, Enigo, Mouse, Settings};
use rand::Rng;
use rhai::{exported_module, Engine};

use crate::input::mouse_button::{MouseButton, MouseButtonModule};

use super::screen_size::{ScreenSize, ScreenSizeModule};

// TODO: Export metadata for the scripting API to use in the language server

#[derive(Debug, Clone)]
pub struct RhaiInterface {
    engine: Arc<Mutex<Engine>>,
    enigo: Arc<Mutex<Enigo>>,
    device: DeviceState,
    rng: rand::rngs::ThreadRng,
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
    pub fn test_click_at(&mut self) {
        let engine = self.engine.lock().unwrap();

        let script = r#"
            let t = new_engine();
                        
            // Click at with the specified mouse button
            //let mb = MouseButton::Left;
            //t.click_at(800, 600, mb);

            // Add the coordinates to the current mouse position
            //t.add_position(100, 100);

            // Print the screen size
            let screen_size = t.get_screen_size();
            print(screen_size.to_string());
            print(screen_size.center().to_string());
        "#;
        engine.run(script).unwrap();

        //let script = engine.compile(script).unwrap();
        //engine.run_ast(&script).unwrap();
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

        // Register the functions
        engine.register_fn("new_engine", RhaiInterface::new);

        // TODO: Add duration/lerp and key chord versions
        engine
            .register_fn("click_at", RhaiInterface::click_at)
            .register_fn("move_mouse", RhaiInterface::move_mouse_to)
            .register_fn("add_position", RhaiInterface::add_position)
            .register_fn("drag_to", RhaiInterface::drag_to)
            .register_fn("drag_from_to", RhaiInterface::drag_from_to);

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

    fn get_screen_size(&mut self) -> ScreenSize {
        // TODO: Cache the result
        let enigo = self.enigo.lock().unwrap();
        let screen_size = enigo
            .main_display()
            .unwrap_or_else(|_| panic!("Failed to get screen size."));

        screen_size.into()
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
