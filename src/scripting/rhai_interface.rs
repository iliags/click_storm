#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use device_query::DeviceState;
use enigo::{Button, Enigo, Mouse, Settings};
use rhai::{exported_module, Engine};

use crate::input::mouse_button::{MouseButton, MouseButtonModule};

use super::screen_size::{ScreenSize, ScreenSizeModule};

#[derive(Debug, Clone)]
pub struct RhaiInterface {
    engine: Arc<Mutex<Engine>>,
    enigo: Arc<Mutex<Enigo>>,
    device: DeviceState,
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
        engine.register_fn("click_at", RhaiInterface::click_at);

        // TODO: Add duration/lerp versions
        engine.register_fn("move_mouse", RhaiInterface::move_mouse_to);
        engine.register_fn("add_position", RhaiInterface::add_position);
        engine.register_fn("drag_to", RhaiInterface::drag_to);
        engine.register_fn("drag_from_to", RhaiInterface::drag_from_to);

        engine.register_fn("get_screen_size", RhaiInterface::get_screen_size);
    }

    /**************************************************************************
     * Scripting API
     **************************************************************************/

    /// Click at the specified coordinates with the specified mouse button.
    fn click_at(&mut self, x: i32, y: i32, mouse_button: MouseButton) {
        let mut enigo = self.enigo.lock().unwrap();
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
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
}
